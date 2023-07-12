include <common.scad>;

module primary_coil_form()
{
  union()
  {
    // primary coil holder
    /*coil_raise = COIL_RAISE - THICKNESS;
    post_height = COIL_HEIGHT + THICKNESS + coil_raise;
    difference()
    {
      union()
      {
        post_raise = (COIL_HEIGHT / 2) + (THICKNESS / 2) - (coil_raise / 2) + coil_raise;

        // holed posts
        for(i = [1 : posts - 1])
        {
          // bar
          rotate([0, 0, i * (360 / posts)])
          translate([COIL_DIAMETER / 2, 0, post_raise])
          cube([THICKNESS * 2, THICKNESS, post_height], center = true);

          // top
          rotate([0, 0, i * 45])
          translate([COIL_DIAMETER / 2, THICKNESS / 2, post_height])
          rotate([90, 0, 0])
          cylinder(d = THICKNESS * 2, h = THICKNESS);
        }

        // terminal post
        translate([(COIL_DIAMETER / 2) + THICKNESS, 0, post_raise])
        cube([THICKNESS, THICKNESS * 2, post_height], center = true);

        // and top
        translate([(COIL_DIAMETER / 2) + (THICKNESS / 2), 0, post_height])
        rotate([90, 0, 90])
        cylinder(d = THICKNESS * 2, h = THICKNESS);
      }

      translate([0, 0, coil_raise])
      {
        // coil
        for(t = [0 : (TURNS - 1)])
        {
          turn_height = COIL_HEIGHT / TURNS;
          step = 5;
          for(a = [0 : step : (360 - step)])
          {
            translate([0, 0, t * turn_height])
            line3D(spiral(COIL_DIAMETER / 2, a, turn_height), spiral(COIL_DIAMETER / 2, a + step, turn_height), WIRE_DIAMETER);
          }
        }

        // top wire
        translate([COIL_DIAMETER / 2, 0, COIL_HEIGHT])
        rotate([0, 90, 0])
        cylinder(h = THICKNESS * 10, d = WIRE_DIAMETER);

        // bottom wire
        translate([COIL_DIAMETER / 2, 0, 0])
        rotate([0, 90, 0])
        cylinder(h = THICKNESS * 10, d = WIRE_DIAMETER);
      }
    }*/

    // lower disk
    difference()
    {
      // shell
      cylinder(d = COIL_DIAMETER + (THICKNESS * 4), h = COIL_HOLDER_RAISE - THICKNESS - THICKNESS);
      // hollow
      cylinder(d = COIL_DIAMETER - (THICKNESS * 2), h = COIL_HOLDER_RAISE);
      // hole
      rotate([0, 0, 15])
      translate([(COIL_DIAMETER / 2) - (THICKNESS * 2), 0, SECONDARY_HOLE_RAISE])
      rotate([0, 90, 0])
      cylinder(h = THICKNESS * 10, d = WIRE_DIAMETER);

      //post_raise = (COIL_HEIGHT / 2) + (THICKNESS / 2) - (coil_raise / 2) + coil_raise;

      // holed post slots
      for(i = [1 : PRIMARY_COIL_POSTS - 1])
      {
        rotate([0, 0, i * (360 / PRIMARY_COIL_POSTS)])
        translate([COIL_DIAMETER / 2, 0, ((COIL_HOLDER_RAISE - THICKNESS - THICKNESS) / 2) + THICKNESS])
        cube([THICKNESS * 2.1, THICKNESS * 1.1, COIL_HOLDER_RAISE - THICKNESS - THICKNESS], center = true);
      }

      // terminal post slot
      translate([(COIL_DIAMETER / 2) + (THICKNESS * 1.5), 0, ((COIL_HOLDER_RAISE - THICKNESS - THICKNESS) / 2) + THICKNESS])
      cube([THICKNESS * 1.1, THICKNESS * 2.1, COIL_HOLDER_RAISE - THICKNESS - THICKNESS], center = true);
    }

    // bolt holes
    bolt_holes = 4;
    disk_edge_radius = (COIL_DIAMETER / 2) + THICKNESS;
    for(i = [1 : bolt_holes])
    {
      bolt_offset = ((NUT_DIAMETER + THICKNESS) / 2) + THICKNESS;
      rotate([0, 0, (i * (360 / bolt_holes)) + (360 / (bolt_holes * 2))])
      translate([disk_edge_radius + bolt_offset, 0, 0])
      difference()
      {
        union()
        {
          translate([-THICKNESS * 2, 0, (THICKNESS / 2)])
          cube([bolt_offset + THICKNESS, NUT_DIAMETER + THICKNESS, THICKNESS], center = true);
          cylinder(h = THICKNESS, d = NUT_DIAMETER + THICKNESS);
        }
        cylinder(h = THICKNESS, d = BOLT_DIAMETER_SMALL);
      }
    }
  }
}