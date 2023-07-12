include <common.scad>;

union()
{
  translate([0, 0, PLUG_THICKNESS])
  coil_form_insert();

  difference()
  {
    cylinder(h = PLUG_THICKNESS, d = COIL_FORM_OD + (PLUG_THICKNESS * 4));
    cylinder(h = PLUG_THICKNESS, d = BOLT_DIAMETER);
    translate([(COIL_FORM_OD + (PLUG_THICKNESS * 4)) / 2, 0, 0])
    cube([PLUG_THICKNESS * 2, PLUG_THICKNESS / 2, PLUG_THICKNESS * 2], true);
  }

  translate([0, 0, PLUG_THICKNESS])
  difference()
  {
    cylinder(h = NUT_THICKNESS, d = NUT_DIAMETER + PLUG_THICKNESS);
    cylinder(h = NUT_THICKNESS, d = NUT_DIAMETER, $fn = 6);
  }
}