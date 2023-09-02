module.exports = {
  type: "Program",
  loc: {
    start: {
      line: 1,
      column: 0,
    },
    end: {
      line: 1,
      column: 0,
    },
  },
  range: [0, 56],
  body: [
    {
      type: "ExpressionStatement",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [0, 23],
      expression: {
        type: "CallExpression",
        loc: {
          start: {
            line: 1,
            column: 0,
          },
          end: {
            line: 1,
            column: 0,
          },
        },
        range: [0, 22],
        callee: {
          type: "MemberExpression",
          loc: {
            start: {
              line: 1,
              column: 0,
            },
            end: {
              line: 1,
              column: 0,
            },
          },
          range: [0, 11],
          object: {
            type: "Identifier",
            loc: {
              start: {
                line: 1,
                column: 0,
              },
              end: {
                line: 1,
                column: 0,
              },
            },
            range: [0, 7],
            name: "console",
          },
          property: {
            type: "Identifier",
            loc: {
              start: {
                line: 1,
                column: 0,
              },
              end: {
                line: 1,
                column: 0,
              },
            },
            range: [8, 11],
            name: "log",
          },
          computed: false,
        },
        arguments: [
          {
            type: "Literal",
            loc: {
              start: {
                line: 1,
                column: 0,
              },
              end: {
                line: 1,
                column: 0,
              },
            },
            range: [12, 21],
            value: "hello, ",
            raw: '"hello, "',
          },
        ],
      },
    },
    {
      type: "DebuggerStatement",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [24, 33],
    },
    {
      type: "ExpressionStatement",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [34, 56],
      expression: {
        type: "CallExpression",
        loc: {
          start: {
            line: 1,
            column: 0,
          },
          end: {
            line: 1,
            column: 0,
          },
        },
        range: [34, 55],
        callee: {
          type: "MemberExpression",
          loc: {
            start: {
              line: 1,
              column: 0,
            },
            end: {
              line: 1,
              column: 0,
            },
          },
          range: [34, 45],
          object: {
            type: "Identifier",
            loc: {
              start: {
                line: 1,
                column: 0,
              },
              end: {
                line: 1,
                column: 0,
              },
            },
            range: [34, 41],
            name: "console",
          },
          property: {
            type: "Identifier",
            loc: {
              start: {
                line: 1,
                column: 0,
              },
              end: {
                line: 1,
                column: 0,
              },
            },
            range: [42, 45],
            name: "log",
          },
          computed: false,
        },
        arguments: [
          {
            type: "Literal",
            loc: {
              start: {
                line: 1,
                column: 0,
              },
              end: {
                line: 1,
                column: 0,
              },
            },
            range: [46, 54],
            value: "world!",
            raw: '"world!"',
          },
        ],
      },
    },
  ],
  tokens: [
    {
      type: "Identifier",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [0, 7],
      value: "console",
    },
    {
      type: "Punctuator",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [7, 8],
      value: ".",
    },
    {
      type: "Identifier",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [8, 11],
      value: "log",
    },
    {
      type: "Punctuator",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [11, 12],
      value: "(",
    },
    {
      type: "String",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [12, 21],
      value: '"hello, "',
    },
    {
      type: "Punctuator",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [21, 22],
      value: ")",
    },
    {
      type: "Punctuator",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [22, 23],
      value: ";",
    },
    {
      type: "Keyword",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [24, 32],
      value: "debugger",
    },
    {
      type: "Punctuator",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [32, 33],
      value: ";",
    },
    {
      type: "Identifier",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [34, 41],
      value: "console",
    },
    {
      type: "Punctuator",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [41, 42],
      value: ".",
    },
    {
      type: "Identifier",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [42, 45],
      value: "log",
    },
    {
      type: "Punctuator",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [45, 46],
      value: "(",
    },
    {
      type: "String",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [46, 54],
      value: '"world!"',
    },
    {
      type: "Punctuator",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [54, 55],
      value: ")",
    },
    {
      type: "Punctuator",
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 0,
        },
      },
      range: [55, 56],
      value: ";",
    },
  ],
  comments: [],
};
