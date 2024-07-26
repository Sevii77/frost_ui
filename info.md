## Requirements
svg:
- AXIS for font
- Miedinger for gil font

## Guidelines
Outlines:
	Color Layer: "Background Color"
	Stroke Color (HSLA): 0, 0, 70, 100
	Stroke Size: 2px (4px if standalone window)

Colors (Dedicated):
	X 100 60-70 (HSL)
	Can be broken if it looks good

Colors Background:
	0 0 90 (HSL) for backgrounds
	0 0 100 (HSL) for elements

Transparency for elements with background color:
	0 0 100 60 (HSLA) (overlayed onto existing element with bg)
	Can be broken if it looks good

Transparency for elements with (0, 0, 5) color:
	0 0 5 60 (HSLA)

Stroke color for icons (colored): (TODO: update older texture to use this, its a mess atm)
	0 0 5 (HSL)

Corner Size (Curved Style):
	Standalone: 22px
	Other: 10px

## Bitmaps
I forgot to update this, so this is outdated (by a large amount)

The following files still use bitmaps to some degree,
Custom icons are also in order for them:
- ui/uld/WindowA_Button
- ui/uld/ConfigSystem (this likely wont change due to accesability)
- ui/uld/CharacterGearSet
- ui/uld/Journal_Detail (a lot)
- ui/uld/Character
- ui/uld/ArmouryBoard
- ui/uld/ToggleButton

## Todo
- Convert all non background elements that use background color as outline to use secondary color for it instead
- ULD composite support, that way we can easily mass edit colors with customisation support (closest color will need to be selected since there is a limited selection)