const {
  RobotPosition,
  robot_position_to_string,
} = require("./pkg/ricochet_robots");

const position = new RobotPosition(10, 10);
console.log(robot_position_to_string(position));
