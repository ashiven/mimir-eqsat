#include <cstdint>

#include <mim/plug/eqsat/eqsat.h>
#include <mim/plug/eqsat/phase/rewrite_slotted.h>

#include "mim/def.h"
#include "mim/driver.h"

#include "mim/plug/eqsat/autogen.h"

namespace mim::plug::eqsat {

void RewriteSlotted::start() {
    auto [rulesets, cost_fn] = import_config();

    // We are assuming that the core plugin and its backends have been loaded at this point
    // because the 'eqsat' plugin declared it as a dependency via 'plugin core;'
    std::ostringstream sexpr;
    driver().backend("sexpr-slotted-typed")(old_world(), sexpr);

    if (DEBUG) std::cout << sexpr.str() << "\n";

    auto rec_exprs = eqsat_slotted(sexpr.str(), rulesets, cost_fn);

    if (DEBUG) std::cout << pretty_ffi(rec_exprs, 80).c_str() << "\n";

    init(rec_exprs);
    convert(rec_exprs);

    swap(old_world(), new_world());
}

std::pair<rust::Vec<RuleSet>, CostFn> RewriteSlotted::import_config() {
    // Internalize config lambdas (with signature [] -> %eqsat.Ruleset | %eqsat.CostFun | %eqsat.Impl)
    DefVec lams;
    for (auto def : old_world().externals().mutate()) {
        if (auto lam = def->isa<Lam>()) {
            if (Axm::isa<eqsat::Ruleset>(lam->ret_dom()) || Axm::isa<eqsat::CostFun>(lam->ret_dom())
                || Axm::isa<eqsat::Impl>(lam->ret_dom())) {
                lams.push_back(lam);
                def->internalize();
            }
        }
    }

    // Import rulesets and cost function from config lambdas
    rust::Vec<RuleSet> rulesets;
    CostFn cost_fn = CostFn::AstSize;
    for (auto lam : lams) {
        auto body = lam->as<Lam>()->body();
        if (auto body_app = body->isa<App>()) {
            if (auto ruleset_config = Axm::isa<eqsat::rulesets>(body_app->arg())) {
                for (auto ruleset : ruleset_config->args())
                    if (Axm::isa<eqsat::standard>(ruleset))
                        rulesets.push_back(RuleSet::Standard);
                    else if (Axm::isa<eqsat::rise>(ruleset))
                        rulesets.push_back(RuleSet::Rise);
                    else
                        assert(false && "Provided ruleset does not exist for slotted");

            } else if (Axm::isa<eqsat::AstSize>(body_app->arg())) {
                cost_fn = CostFn::AstSize;
            } else if (Axm::isa<eqsat::slotted>(body_app->arg()) || Axm::isa<eqsat::egg>(body_app->arg())) {
                continue;
            } else {
                assert(false && "Invalid config value provided for slotted");
            }
        }
    }

    return {rulesets, cost_fn};
}

const Def* RewriteSlotted::create_type(RecExprFFI type_) {
    auto root_id = type_.nodes.size() - 1;

    // Save current state
    auto saved_cache      = cache();
    auto saved_loc        = loc();
    auto saved_visits     = depth_visits();
    auto saved_scope_tree = scope_tree();
    auto saved_scope      = scope();
    auto saved_nodes      = nodes();

    // Prepare environment
    auto temp_cache      = Cache{};
    auto temp_scope_tree = ScopeTree{};
    reset_loc();
    reset_depth_visits();
    set_cache(&temp_cache);
    set_scope_tree(&temp_scope_tree);
    set_scope(loc());
    set_nodes(type_.nodes);

    // Convert type
    init(root_id);
    auto res = convert(root_id);

    // Restore state
    set_cache(saved_cache);
    set_loc(saved_loc);
    set_depth_visits(saved_visits);
    set_scope_tree(saved_scope_tree);
    set_scope(saved_scope);
    set_nodes(saved_nodes);

    return res;
}

void RewriteSlotted::init(rust::Vec<RecExprFFI> rec_exprs) {
    size_t rec_expr_id = 0;
    for (auto rec_expr : rec_exprs) {
        reset_loc();
        reset_depth_visits();

        set_cache(rec_expr_id);
        set_nodes(rec_expr.nodes);
        set_scope_tree(rec_expr_id);
        set_scope(loc());

        auto root_id = nodes().size() - 1;
        init(root_id, true);

        rec_expr_id++;
    }
}

const Def* RewriteSlotted::init(uint32_t id, bool init_lookahead /* = false */) {
    auto node = get_node_unsafe(id);
    enter_scope(node);

    const Def* res = cache_get(id);
    switch (node.kind) {
        case MimKind::Axm: res = init_axm(id, node); break;
        case MimKind::Root: res = init_root(id, node); break;
        case MimKind::Let: res = init_let(id, node); break;
        default: break;
    }

    // If init_lookahead=true we implicitly call init_lam/pi/sigma/arr
    // via their surrounding let/root binders init_let/init_root
    // We don't need to initialize with a lookahead when creating types or let/root
    // definition subterms because we know that they do not contain
    // let-bindings or root-bindings which require this lookahead initialization.
    if (!init_lookahead) {
        switch (node.kind) {
            case MimKind::Lam: res = init_lam(id, node); break;
            case MimKind::Pi: res = init_pi(id, node); break;
            case MimKind::Sigma: res = init_sigma(id, node); break;
            case MimKind::Arr: res = init_arr(id, node); break;
            default: break;
        }
    }

    for (uint32_t child : node.children)
        init(child, init_lookahead);

    exit_scope(node, true);
    return cache_set(id, res);
}

const Def* RewriteSlotted::init_lookahead(uint32_t id, NodeFFI node) {
    const Def* def = cache_get(id);
    switch (node.kind) {
        case MimKind::Lam: def = init_lam(id, node); break;
        case MimKind::Pi: def = init_pi(id, node); break;
        case MimKind::Sigma: def = init_sigma(id, node); break;
        case MimKind::Arr: def = init_arr(id, node); break;
        default:
            init(id);
            def = convert(id);
            break;
    }
    return def;
}

// (axm <name> <type>)
const Def* RewriteSlotted::init_axm(uint32_t id, NodeFFI node) {
    if (DEBUG) std::cout << "init - current node(" << id << "): " << node_ffi_str(node).c_str() << " - ";
    auto name = get_symbol(node.children[0]);
    if (DEBUG) std::cout << "\n";
    // TODO: Use create_type instead of convert
    auto type = convert(node.children[1]);

    auto new_axm = new_world().axm(type);
    new_axm->set(name);
    register_axm(name, new_axm);

    if (DEBUG) std::cout << new_axm << "\n";
    return new_axm;
}

// (root <extern> <name> <definition>)
const Def* RewriteSlotted::init_root(uint32_t id, NodeFFI node) {
    if (DEBUG) std::cout << "init - current node(" << id << "): " << node_ffi_str(node).c_str() << " - \n";

    auto name = get_symbol(node.children[1]);

    auto def_id   = node.children[2];
    auto def_node = get_node_unsafe(def_id);
    auto def      = init_lookahead(def_id, def_node);
    def->set(name);
    register_var(name, def);

    if (DEBUG) std::cout << def << "\n";
    return nullptr;
}

// (let $var (scope <definition> <expression>))
const Def* RewriteSlotted::init_let(uint32_t id, NodeFFI node) {
    if (DEBUG) std::cout << "init - current node(" << id << "): " << node_ffi_str(node).c_str() << " - \n";

    auto var_name  = get_slot(id);
    auto var_scope = get_node(MimKind::Scope, node.children[0]);

    enter_scope(var_scope);

    auto def_id   = var_scope.children[0];
    auto def_node = get_node_unsafe(def_id);
    auto def      = init_lookahead(def_id, def_node);
    def->set(var_name);
    register_var(var_name, def);

    exit_scope(var_scope);

    if (DEBUG) std::cout << def << "\n";
    return nullptr;
}

// (lam $var (scope <filter> <body>))
const Def* RewriteSlotted::init_lam(uint32_t id, NodeFFI node) {
    if (DEBUG) std::cout << "init - current node(" << id << "): " << node_ffi_str(node).c_str() << " - \n";

    auto pi_type = create_type(node.type_)->as<Pi>();
    auto new_lam = new_world().mut_lam(pi_type);

    auto var_name = get_slot(id);
    auto var      = new_lam->var();
    var->set(var_name);

    auto var_scope = get_node(MimKind::Scope, node.children[0]);
    enter_scope(var_scope);
    register_var(var_name, var);
    exit_scope(var_scope);

    return cache_set(id, new_lam);
}

// (pi $var (scope <dom> <codom>))
const Def* RewriteSlotted::init_pi(uint32_t id, NodeFFI node) {
    if (DEBUG) std::cout << "init - current node(" << id << "): " << node_ffi_str(node).c_str() << " - \n";

    auto new_pi = new_world().mut_pi(new_world().type());

    auto var_name = get_slot(id);
    auto var      = new_pi->var();
    var->set(var_name);

    auto var_scope = get_node(MimKind::Scope, node.children[0]);
    enter_scope(var_scope);
    register_var(var_name, var);
    exit_scope(var_scope);

    return cache_set(id, new_pi);
}

// (sigma $var (scope <elem-cons> nil))
const Def* RewriteSlotted::init_sigma(uint32_t id, NodeFFI node) {
    if (DEBUG) std::cout << "init - current node(" << id << "): " << node_ffi_str(node).c_str() << " - \n";

    auto var_scope = get_node(MimKind::Scope, node.children[0]);
    auto elem_cons = get_cons_flat(var_scope.children[0]);
    auto size      = elem_cons.size();

    auto new_sigma = new_world().mut_sigma(size);

    auto var_name = get_slot(id);
    auto var      = new_sigma->var();
    var->set(var_name);

    enter_scope(var_scope);
    register_var(var_name, var);
    exit_scope(var_scope);

    return cache_set(id, new_sigma);
}

// (arr $var (scope <arity> <body>))
const Def* RewriteSlotted::init_arr(uint32_t id, NodeFFI node) {
    if (DEBUG) std::cout << "init - current node(" << id << "): " << node_ffi_str(node).c_str() << " - \n";

    auto new_arr = new_world().mut_arr();

    auto var_scope = get_node(MimKind::Scope, node.children[0]);
    auto arity     = convert(var_scope.children[0]);
    new_arr->set_arity(arity);

    auto var_name = get_slot(id);
    auto var      = new_arr->var();
    var->set(var_name);

    enter_scope(var_scope);
    register_var(var_name, var);
    exit_scope(var_scope);

    return cache_set(id, new_arr);
}

void RewriteSlotted::convert(rust::Vec<RecExprFFI> rec_exprs) {
    size_t rec_expr_id = 0;
    for (auto rec_expr : rec_exprs) {
        reset_loc();
        reset_depth_visits();

        set_cache(rec_expr_id);
        set_nodes(rec_expr.nodes);
        set_scope_tree(rec_expr_id);
        set_scope(loc());

        auto root_id = nodes().size() - 1;
        convert(root_id);

        rec_expr_id++;
    }
}

const Def* RewriteSlotted::convert(uint32_t id) {
    auto node = get_node_unsafe(id);

    enter_scope(node);

    for (uint32_t child : node.children)
        convert(child);

    // We don't want to short-circuit through the cache here
    // because some Def's (mutables) need to be converted after
    // they have already been created in init().
    // We could later short-circuit for all other terms except
    // those that need to be revisited..
    const Def* res = cache_get(id);

    if (DEBUG) std::cout << "convert - current node(" << id << "): " << node_ffi_str(node).c_str() << " - ";
    switch (node.kind) {
        case MimKind::Root: res = convert_root(id, node); break;
        case MimKind::Let: res = convert_let(id, node); break;
        case MimKind::Lam: res = convert_lam(id, node); break;
        case MimKind::App: res = convert_app(id, node); break;
        case MimKind::Var: res = convert_var(id, node); break;
        case MimKind::Lit: res = convert_lit(id, node); break;
        case MimKind::Pack: res = convert_pack(id, node); break;
        case MimKind::Tuple: res = convert_tuple(id, node); break;
        case MimKind::Extract: res = convert_extract(id, node); break;
        case MimKind::Insert: res = convert_insert(id, node); break;
        case MimKind::Inj: res = convert_inj(id, node); break;
        case MimKind::Merge: res = convert_merge(id, node); break;
        case MimKind::Match: res = convert_match(id, node); break;
        case MimKind::Proxy: res = convert_proxy(id, node); break;
        case MimKind::Join: res = convert_join(id, node); break;
        case MimKind::Meet: res = convert_meet(id, node); break;
        case MimKind::Bot: res = convert_bot(id, node); break;
        case MimKind::Top: res = convert_top(id, node); break;
        case MimKind::Arr: res = convert_arr(id, node); break;
        case MimKind::Sigma: res = convert_sigma(id, node); break;
        case MimKind::Cn: res = convert_cn(id, node); break;
        case MimKind::Pi: res = convert_pi(id, node); break;
        case MimKind::Idx: res = convert_idx(id, node); break;
        case MimKind::Hole: res = convert_hole(id, node); break;
        case MimKind::Type: res = convert_type(id, node); break;
        case MimKind::Num: res = convert_num(id, node); break;
        case MimKind::Symbol: res = convert_symbol(id, node); break;
        default: break;
    }

    if (res)
        if (auto mut = res->isa_mut()) mut->immutabilize();

    if (DEBUG_SCOPES && node.kind == MimKind::Scope) std::cout << "\n";
    exit_scope(node, true);

    if (DEBUG) std::cout << res << "\n";
    return cache_set(id, res);
}

// (root <extern> <name> <definition>)
const Def* RewriteSlotted::convert_root(uint32_t id, NodeFFI node) {
    auto is_extern = get_symbol(node.children[0]);
    auto def       = get_def(node.children[1]);

    if (auto lam = def->isa_mut<Lam>()) {
        if (is_extern == "extern") lam->externalize();
    }

    return def;
}

// (let $var (scope <definition> <expression>))
const Def* RewriteSlotted::convert_let(uint32_t id, NodeFFI node) {
    auto var_scope = get_node(MimKind::Scope, node.children[0]);
    enter_scope(var_scope, true);
    auto expr = get_def(var_scope.children[1]);
    exit_scope(var_scope);

    return expr;
}

// (lam $var (scope <filter> <body>))
const Def* RewriteSlotted::convert_lam(uint32_t id, NodeFFI node) {
    auto lam       = get_def(id)->as_mut<Lam>();
    auto var_scope = get_node(MimKind::Scope, node.children[0]);

    enter_scope(var_scope, true);
    auto filter = get_def(var_scope.children[0]);
    auto body   = get_def(var_scope.children[1]);
    if (filter && body) {
        lam->set_filter(filter);
        lam->set_body(body);
    } else {
        lam->set_filter(false);
    }
    exit_scope(var_scope);

    return lam;
}

// (app <callee> <arg>)
const Def* RewriteSlotted::convert_app(uint32_t id, NodeFFI node) {
    auto callee  = get_def(node.children[0]);
    auto arg     = get_def(node.children[1]);
    auto new_app = new_world().app(callee, arg);
    return new_app;
}

// (var $name)
const Def* RewriteSlotted::convert_var(uint32_t id, NodeFFI node) {
    auto var = get_def(id);
    return var;
}

// (lit <val> <type>)
const Def* RewriteSlotted::convert_lit(uint32_t id, NodeFFI node) {
    auto lit_def = get_def(node.children[0]);
    if (lit_def) return lit_def;

    auto lit_val  = get_num(node.children[0]);
    auto lit_type = get_def(node.children[1]);
    auto new_lit  = new_world().lit(lit_type, lit_val);
    return new_lit;
}

// (pack <arity> <body>)
const Def* RewriteSlotted::convert_pack(uint32_t id, NodeFFI node) {
    auto arity    = get_def(node.children[0]);
    auto body     = get_def(node.children[1]);
    auto new_pack = new_world().pack(arity, body);
    return new_pack;
}

// (tuple <elem-cons>)
const Def* RewriteSlotted::convert_tuple(uint32_t id, NodeFFI node) {
    auto elem_ids = get_cons_flat(node.children[0]);

    DefVec elems;
    for (auto elem_id : elem_ids) {
        auto elem = get_def(elem_id);
        elems.push_back(elem);
    }
    auto new_tuple = new_world().tuple(elems);
    return new_tuple;
}

// (extract <tuple> <index>)
const Def* RewriteSlotted::convert_extract(uint32_t id, NodeFFI node) {
    auto tuple       = get_def(node.children[0]);
    auto index       = get_def(node.children[1]);
    auto new_extract = new_world().extract(tuple, index);
    return new_extract;
}

// (ins <tuple> <index> <value>)
const Def* RewriteSlotted::convert_insert(uint32_t id, NodeFFI node) {
    auto tuple      = get_def(node.children[0]);
    auto index      = get_def(node.children[1]);
    auto value      = get_def(node.children[2]);
    auto new_insert = new_world().insert(tuple, index, value);
    return new_insert;
}

// (inj <type> <value>)
const Def* RewriteSlotted::convert_inj(uint32_t id, NodeFFI node) {
    auto type    = get_def(node.children[0]);
    auto value   = get_def(node.children[1]);
    auto new_inj = new_world().inj(type, value);
    return new_inj;
}

// (merge <type> <value-cons>)
const Def* RewriteSlotted::convert_merge(uint32_t id, NodeFFI node) {
    auto type = get_def(node.children[0]);

    auto value_ids = get_cons_flat(node.children[1]);
    DefVec values;
    for (auto value_id : value_ids) {
        auto value = get_def(value_id);
        values.push_back(value);
    }
    auto new_merge = new_world().merge(type, values);
    return new_merge;
}

// (match <op-cons>)
const Def* RewriteSlotted::convert_match(uint32_t id, NodeFFI node) {
    auto op_ids = get_cons_flat(node.children[0]);

    DefVec ops;
    for (auto op_id : op_ids) {
        auto op = get_def(op_id);
        ops.push_back(op);
    }
    auto new_match = new_world().match(ops);
    return new_match;
}

// (proxy <type> <pass> <tag> <op-cons>)
const Def* RewriteSlotted::convert_proxy(uint32_t id, NodeFFI node) {
    auto type = get_def(node.children[0]);
    auto pass = get_num(node.children[1]);
    auto tag  = get_num(node.children[2]);

    auto op_ids = get_cons_flat(node.children[3]);
    DefVec ops;
    for (auto op_id : op_ids) {
        auto op = get_def(op_id);
        ops.push_back(op);
    }
    auto new_proxy = new_world().proxy(type, ops, pass, tag);
    return new_proxy;
}

// (join <type-cons>)
const Def* RewriteSlotted::convert_join(uint32_t id, NodeFFI node) {
    auto type_ids = get_cons_flat(node.children[0]);

    DefVec types;
    for (auto type_id : type_ids) {
        auto type = get_def(type_id);
        types.push_back(type);
    }
    auto new_join = new_world().join(types);
    return new_join;
}

// (meet <type-cons>)
const Def* RewriteSlotted::convert_meet(uint32_t id, NodeFFI node) {
    auto type_ids = get_cons_flat(node.children[0]);

    DefVec types;
    for (auto type_id : type_ids) {
        auto type = get_def(type_id);
        types.push_back(type);
    }
    auto new_meet = new_world().meet(types);
    return new_meet;
}

// (bot <type>)
const Def* RewriteSlotted::convert_bot(uint32_t id, NodeFFI node) {
    auto type    = get_def(node.children[0]);
    auto new_bot = new_world().bot(type);
    return new_bot;
}

// (top <type>)
const Def* RewriteSlotted::convert_top(uint32_t id, NodeFFI node) {
    auto type    = get_def(node.children[0]);
    auto new_top = new_world().top(type);
    return new_top;
}

// (arr $var (scope <arity> <body>))
const Def* RewriteSlotted::convert_arr(uint32_t id, NodeFFI node) {
    auto arr = get_def(id)->as_mut<Arr>();

    auto var_scope = get_node(MimKind::Scope, node.children[0]);
    enter_scope(var_scope, true);

    auto body = get_def(var_scope.children[1]);

    exit_scope(var_scope);

    // We already converted some mutable pi/sigma/arr that appear in let/root def subterms
    // via init_lookahead and setting them again when they have already been set would result in an error.
    if (!arr->body()) arr->set_body(body);
    return arr;
}

// (sigma $var (scope <type-cons> nil))
const Def* RewriteSlotted::convert_sigma(uint32_t id, NodeFFI node) {
    auto sigma = get_def(id)->as_mut<Sigma>();

    auto var_scope = get_node(MimKind::Scope, node.children[0]);
    enter_scope(var_scope, true);

    DefVec types;
    auto type_ids = get_cons_flat(var_scope.children[0]);
    for (auto type_id : type_ids) {
        auto type = get_def(type_id);
        types.push_back(type);
    }

    exit_scope(var_scope);

    if (!sigma->is_set()) sigma->set(types);
    return sigma;
}

// (cn <domain>)
const Def* RewriteSlotted::convert_cn(uint32_t id, NodeFFI node) {
    auto domain = get_def(node.children[0]);
    auto new_cn = new_world().cn(domain);
    return new_cn;
}

// (pi $var (scope <domain> <codomain>))
const Def* RewriteSlotted::convert_pi(uint32_t id, NodeFFI node) {
    auto pi = get_def(id)->as_mut<Pi>();

    auto var_scope = get_node(MimKind::Scope, node.children[0]);
    enter_scope(var_scope, true);

    auto domain   = get_def(var_scope.children[0]);
    auto codomain = get_def(var_scope.children[1]);

    exit_scope(var_scope);

    if (!pi->is_set()) pi->set(domain, codomain);
    return pi;
}

// (idx <size>)
const Def* RewriteSlotted::convert_idx(uint32_t id, NodeFFI node) {
    auto size    = get_def(node.children[0]);
    auto new_idx = new_world().type_idx(size);
    return new_idx;
}

// (hole <type>)
const Def* RewriteSlotted::convert_hole(uint32_t id, NodeFFI node) {
    auto type_    = get_def(node.children[0]);
    auto new_hole = new_world().mut_hole(type_);
    return new_hole;
}

// (type <level>)
const Def* RewriteSlotted::convert_type(uint32_t id, NodeFFI node) {
    auto level    = get_def(node.children[0]);
    auto new_type = new_world().type(level);
    return new_type;
}

// <u64>
const Def* RewriteSlotted::convert_num(uint32_t id, NodeFFI node) { return nullptr; }

// <string>
const Def* RewriteSlotted::convert_symbol(uint32_t id, NodeFFI node) {
    auto def = get_def(id);
    return def;
}

} // namespace mim::plug::eqsat
