include <common.scad>;

POST_SHORTEN_BY = THICKNESS;

module line3D(p1, p2, thickness)
{
  hull()
  {
    translate(p1)
    sphere(d = thickness, $fn = 10);

    translate(p2)
    sphere(d = thickness, $fn = 10);
  }
}

function spiral(radius, angle, total_rise) = [radius * cos(angle), radius * sin(angle), total_rise * (angle / 360)];

module coil()
{
  translate([0, 0, COIL_RAISE - POST_SHORTEN_BY])
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
}

// primary coil holder
post_height = COIL_HEIGHT + COIL_RAISE - POST_SHORTEN_BY;
union()
{
  // holed posts
  for(i = [1 : PRIMARY_COIL_POSTS - 1])
  {
    translate([(i * (THICKNESS * 2.5)) - (COIL_DIAMETER / 2), 0, 0])
    rotate([0, 0, -i * (360 / PRIMARY_COIL_POSTS)])
    difference()
    {
      rotate([0, 0, i * (360 / PRIMARY_COIL_POSTS)])
      translate([COIL_DIAMETER / 2, 0, 0])
      union()
      {
        // bar
        translate([0, 0, post_height / 2])
        cube([THICKNESS * 2, THICKNESS, post_height], center = true);

        // top
        translate([0, THICKNESS / 2, post_height])
        rotate([90, 0, 0])
        cylinder(d = THICKNESS * 2, h = THICKNESS);
      }

      coil();
    }
  }

  rotate([0, 0, 90])
  translate([-((COIL_DIAMETER / 2) + THICKNESS), 0, 0])
  difference()
  {
    translate([(COIL_DIAMETER / 2) + THICKNESS, 0, 0])
    union()
    {
      // terminal post
      translate([0, 0, post_height / 2])
      cube([THICKNESS, THICKNESS * 2, post_height], center = true);

      // and top
      translate([-THICKNESS / 2, 0, post_height])
      rotate([90, 0, 90])
      cylinder(d = THICKNESS * 2, h = THICKNESS);
    }

    coil();
  }
}