const {
  RobotPosition,
} = require("./pkg/ricochet_robots");

const position = new RobotPosition(10, 10);
console.log(position.to_string());
