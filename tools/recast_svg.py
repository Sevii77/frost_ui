import math

# defs = """<clipPath clipPathUnits="userSpaceOnUse" id="clipPath{0}">
# 	<path style="opacity:1;mix-blend-mode:normal;fill:#ffffff;fill-opacity:1;fill-rule:nonzero;stroke:none;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:0.8;stroke-opacity:1;paint-order:normal" id="path{0}" sodipodi:type="arc" sodipodi:cx="44" sodipodi:cy="42" sodipodi:rx="35" sodipodi:ry="35" sodipodi:start="-1.57079632679" sodipodi:end="{1}" sodipodi:arc-type="slice" d="M 44,7 A 35,35 0 0 1 76.496366,29.00053 35,35 0 0 1 68.139173,67.343644 35,35 0 0 1 29.434861,73.82541 L 44,42 Z" />
# </clipPath>"""

# pos offset 88

shapes = """<g id="g{0}" style="display:inline;fill:#ffffff" inkscape:label="{0}" transform="translate({1},{2})">
	<path style="display:inline;mix-blend-mode:normal;fill:#b2b2b2;fill-opacity:0.4;stroke:#b2b2b2;stroke-width:2;stroke-linecap:butt;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:7;stroke-opacity:1;paint-order:normal" id="rect{0}" width="78" height="78" x="5" y="3" ry="7" inkscape:label="frame" clip-path="url(#clipPath{0})" sodipodi:type="rect" inkscape:path-effect="#path-effect{0}" d="m 12,3 h 64 c 3.878,0 7,3.122 7,7 v 64 c 0,3.878 -3.122,7 -7,7 H 12 C 8.122,81 5,77.878 5,74 V 10 C 5,6.122 8.122,3 12,3 Z" />
	<path style="opacity:1;mix-blend-mode:normal;fill:#ffffff;fill-opacity:1;fill-rule:nonzero;stroke:none;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:0.8;stroke-opacity:1;paint-order:normal" id="path{0}" sodipodi:type="arc" sodipodi:cx="44" sodipodi:cy="42" sodipodi:rx="35" sodipodi:ry="35" sodipodi:start="{4}" sodipodi:end="{3}" sodipodi:arc-type="slice" d="M 44,7 A 35,35 0 0 1 76.496366,29.00053 35,35 0 0 1 68.139173,67.343644 35,35 0 0 1 29.434861,73.82541 L 44,42 Z" />
	<path style="display:block;opacity:1;mix-blend-mode:normal;fill:none;fill-opacity:1;fill-rule:nonzero;stroke:#ffffff;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:0.8;stroke-opacity:1;paint-order:normal;filter:url(#filter125)" id="path{0}" sodipodi:type="arc" sodipodi:cx="44" sodipodi:cy="42" sodipodi:rx="35" sodipodi:ry="35" sodipodi:start="{4}" sodipodi:end="{3}" sodipodi:arc-type="slice" d="M 79,42 A 35,35 0 0 1 62.910581,71.451484 35,35 0 0 1 29.434861,73.82541 L 44,42 Z" />
</g>"""

# all_defs = ""
all_shapes = ""
for i in range(1, 81):
	rot_start = math.pi * 1.5
	rot = i / 40 * math.pi + rot_start
	
	# d = defs.format(i, rot)
	shape = shapes.format(i, 88 * (i % 9), 96 * math.floor(i / 9), rot, rot_start)
	
	# all_defs = all_defs + "\n" + d
	all_shapes = all_shapes + "\n" + shape

# with open("./defs", "w") as f:
# 	f.write(all_defs)
with open("./shapes", "w") as f:
	f.write(all_shapes)