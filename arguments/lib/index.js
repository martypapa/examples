const {
  printFunction,
  add1ToArgument,
  getArgsLen,
  argsOpt,
  defaultArgs
} = require('../native/index.node');

console.log(
  printFunction(() => {}),
  add1ToArgument(1),
  getArgsLen(1, 2, 3),
  argsOpt(1),
  defaultArgs()
);

module.exports = require('../native/index.node');
