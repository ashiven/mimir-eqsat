# A vibe-coded script to quickly extract the scope locations from
# an sexpr in slotted-egraphs format (useful for debugging)

import re
from collections import defaultdict

sexpr = open("input.txt").read()

# ------------------------------------------------------------
# Tokenize
# ------------------------------------------------------------

tokens = re.findall(r"\(|\)|[^\s()]+", sexpr)

# ------------------------------------------------------------
# Parse
# ------------------------------------------------------------


def parse(tokens):
    stack = []
    curr = []

    for tok in tokens:
        if tok == "(":
            stack.append(curr)
            new = []
            curr.append(new)
            curr = new
        elif tok == ")":
            curr = stack.pop()
        else:
            curr.append(tok)

    return curr[0]


tree = parse(tokens)

# ------------------------------------------------------------
# Global counters
# ------------------------------------------------------------

depth_offsets = defaultdict(int)

# var name -> list[(depth, offset)]
locations = defaultdict(list)

# ------------------------------------------------------------
# Traverse
# ------------------------------------------------------------


def walk(node, depth=0):

    if not isinstance(node, list) or not node:
        return

    head = node[0]

    if head in ("lam", "let") and len(node) >= 3:

        var = node[1]

        offset = depth_offsets[depth]

        locations[var].append((depth, offset))

        depth_offsets[depth] += 1

        walk(node[2], depth + 1)

        return

    for child in node:
        walk(child, depth)


walk(tree)

# ------------------------------------------------------------
# Print
# ------------------------------------------------------------

for var, locs in locations.items():
    for loc in locs:
        print(f"{var} -> {loc}")
