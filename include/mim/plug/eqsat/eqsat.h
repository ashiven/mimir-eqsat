#pragma once

#include "mim/world.h"

#include "mim/plug/eqsat/autogen.h"

namespace mim {

// fun extern _impl(): %eqsat.Impl =
//     return <impl>;
void eqsat_impl(World& world, flags_t impl) {
    auto Impl     = world.annex<plug::eqsat::Impl>();
    auto _impl    = world.mut_fun({}, Impl)->set("_impl");
    auto ret      = _impl->var(2, 1);
    auto impl_axm = world.annex(impl);
    _impl->app(false, ret, impl_axm);
    _impl->externalize();
}

// fun extern _cost_fun(): %eqsat.CostFun =
//     return <cost_fun>;
void eqsat_cost_fun(World& world, flags_t cost_fun) {
    auto CostFun      = world.annex<plug::eqsat::CostFun>();
    auto _cost_fun    = world.mut_fun({}, CostFun)->set("_cost_fun");
    auto ret          = _cost_fun->var(2, 1);
    auto cost_fun_axm = world.annex(cost_fun);
    _cost_fun->app(false, ret, cost_fun_axm);
    _cost_fun->externalize();
}

// fun extern _rulesets(): %eqsat.Ruleset =
//     return %eqsat.rulesets (<rulesets>);
void eqsat_rulesets(World& world, std::vector<flags_t> rulesets) {
    auto Ruleset   = world.annex<plug::eqsat::Ruleset>();
    auto _rulesets = world.mut_fun({}, Ruleset)->set("_rulesets");
    auto ret       = _rulesets->var(2, 1);

    DefVec ruleset_axms;
    for (auto ruleset : rulesets) {
        auto ruleset_axm = world.annex(ruleset);
        ruleset_axms.push_back(ruleset_axm);
    }

    auto ruleset_tuple = world.tuple(ruleset_axms);
    auto rulesets_app  = world.call(world.annex<plug::eqsat::rulesets>(), ruleset_tuple);

    _rulesets->app(false, ret, rulesets_app);
    _rulesets->externalize();
}

// fun extern _rules(): %eqsat.Rules =
//     return %eqsat.rules (<rules>);
void eqsat_rules(World& world, DefVec rules) {
    auto Rules  = world.annex<plug::eqsat::Rules>();
    auto _rules = world.mut_fun({}, Rules)->set("_rules");
    auto ret    = _rules->var(2, 1);

    auto rules_tuple = world.tuple(rules);
    auto rules_app   = world.call(world.annex<plug::eqsat::rules>(), rules_tuple);

    _rules->app(false, ret, rules_app);
    _rules->externalize();
}

} // namespace mim
