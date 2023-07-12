include <primary_coil_form.scad>;

difference()
{
  primary_coil_form();
  rotate([0, 0, 180 + 22.5])
  translate([0, -100 * CM, 0])
  cube([200 * CM, 200 * CM, 200 * CM]);
}