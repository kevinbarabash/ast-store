const { parse } = require("@babel/parser");
const traverse = require("@babel/traverse").default;
const { SourceCode } = require("eslint");
const convertAST = require("../node_modules/@babel/eslint-parser/lib/convert/convertAST.cjs");

console.log("traverse = ", traverse);

const noDebugger = require("../node_modules/eslint/lib/rules/no-debugger.js");
const noConsole = require("../node_modules/eslint/lib/rules/no-console.js");

const input = `
console.log("hello, ");
debugger;
console.log("world!");
`;

const ast = convertAST(
  parse(input, {
    sourceType: "module",
    loc: true,
    sourceFilename: "test.js",
    tokens: true,
    ranges: true,
  })
);

console.log(ast);

const sourceCode = new SourceCode(input, ast);

const adaptVisitor = (visitor) => {
  const newVisitor = {};

  for (const [key, value] of Object.entries(visitor)) {
    // TODO: handle cases where both :enter and :exit are present
    if (key.endsWith(":exit")) {
      newVisitor[key.replace(":exit", "")] = {
        exit: (path) => {
          value(path.node);
        },
      };
    } else {
      newVisitor[key] = (path) => {
        value(path.node);
      };
    }
  }

  return newVisitor;
};

const rules = [noDebugger, noConsole];

const runRule = (ast, rule) => {
  const visitor = rule.create({
    report: ({ node, messageId }) => {
      const message = rule.meta.messages[messageId];
      console.log(message);
      console.log("location = ", node.loc);
    },
    options: {}, // TODO: populate with options from .eslintrc.js
    sourceCode,
  });

  traverse(ast, adaptVisitor(visitor));
};

const runRules = (ast, rules) => {
  for (const rule of rules) {
    runRule(ast, rule);
  }
};

runRules(ast, rules);
