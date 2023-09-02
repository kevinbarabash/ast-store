const traverse = require("@babel/traverse").default;
const { SourceCode } = require("eslint");
const evk = require("eslint-visitor-keys");
const eslintScope = require("eslint-scope");

const noDebugger = require("../node_modules/eslint/lib/rules/no-debugger.js");
const noConsole = require("../node_modules/eslint/lib/rules/no-console.js");

const ast = require("./ast.js");
const input = `
console.log("hello, ");
debugger;
console.log("world!");
`;

const rules = [noDebugger, noConsole];

const DEFAULT_ECMA_VERSION = 5;

const analyzeScope = (ast, languageOptions, visitorKeys) => {
  const parserOptions = languageOptions.parserOptions;
  const ecmaFeatures = parserOptions.ecmaFeatures || {};
  const ecmaVersion = languageOptions.ecmaVersion || DEFAULT_ECMA_VERSION;

  return eslintScope.analyze(ast, {
    ignoreEval: true,
    nodejsScope: ecmaFeatures.globalReturn,
    impliedStrict: ecmaFeatures.impliedStrict,
    ecmaVersion: typeof ecmaVersion === "number" ? ecmaVersion : 6,
    sourceType: languageOptions.sourceType || "script",
    childVisitorKeys: visitorKeys || evk.KEYS,
    fallback: evk.getKeys,
  });
};

const runRules = (ast, rules) => {
  const config = {
    text: input,
    ast,
    undefined, // ParserServices | undefined,
    scopeManager: analyzeScope(ast, {
      ecmaVersion: 13, // ES2022
      parserOptions: { ecmaFeatures: { jsx: true } },
    }),
  };
  const sourceCode = new SourceCode(config);

  const visitors = rules.map((rule) => {
    return rule.create({
      report: ({ node, messageId }) => {
        const message = rule.meta.messages[messageId];
        const start = `${node.loc.start.line}:${node.loc.start.column}`;
        const end = `${node.loc.end.line}:${node.loc.end.column}`;
        console.log(`${start} to ${end} - ${message}`);
      },
      options: {}, // TODO: populate with options from .eslintrc.js
      sourceCode,
    });
  });

  traverse(
    // NOTE: this wrapper is required in order for traverse() to visit
    // the 'Program' node at the root of `ast`.
    {
      type: "File",
      program: ast,
    },
    {
      enter(path) {
        const { node } = path;
        node.parent = path.parentPath?.node;
        for (const visitor of visitors) {
          if (node.type in visitor) {
            return visitor[node.type](node);
          } else if (`${node.type}:enter` in visitor) {
            return visitor[`${node.type}:enter`](node);
          }
        }
      },
      exit(path) {
        const { node } = path;
        node.parent = path.parentPath?.node;
        for (const visitor of visitors) {
          if (`${node.type}:exit` in visitor) {
            return visitor[`${node.type}:exit`](node);
          }
        }
      },
    }
  );
};

runRules(ast, rules);
