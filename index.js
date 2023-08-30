const toBabel = require("swc-to-babel");
const { ESLint } = require("eslint");

const swcAst = {
  type: "Module",
  span: { start: 1, end: 22, ctxt: 0 },
  body: [
    {
      type: "ExpressionStatement",
      span: { start: 1, end: 22, ctxt: 0 },
      expression: {
        type: "CallExpression",
        span: { start: 1, end: 21, ctxt: 0 },
        callee: {
          type: "MemberExpression",
          span: { start: 1, end: 12, ctxt: 0 },
          object: {
            type: "Identifier",
            span: { start: 1, end: 8, ctxt: 0 },
            value: "console",
            optional: false,
          },
          property: {
            type: "Identifier",
            span: { start: 9, end: 12, ctxt: 0 },
            value: "log",
            optional: false,
          },
        },
        arguments: [
          {
            spread: null,
            expression: {
              type: "StringLiteral",
              span: { start: 13, end: 20, ctxt: 0 },
              value: "hello",
              raw: '"hello"',
            },
          },
        ],
        typeArguments: null,
      },
    },
  ],
  interpreter: null,
};

const babelAst = toBabel(swcAst, 'console.log("hello");');
console.log(JSON.stringify(babelAst, null, 4));

const main = async () => {
  const eslint = new ESLint();
  const config = await eslint.calculateConfigForFile("./.eslintrc.js");
  console.log(config);
};

main().then(() => {
  console.log("done");
});
