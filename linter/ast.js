module.exports = {
  type: "Program",
  start: 0,
  end: 56,
  loc: {
    start: {
      line: 1,
      column: 0,
    },
    end: {
      line: 3,
      column: 22,
    },
  },
  range: [0, 56],
  body: [
    {
      type: "ExpressionStatement",
      start: 0,
      end: 23,
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 23,
        },
      },
      range: [0, 23],
      expression: {
        type: "CallExpression",
        start: 0,
        end: 22,
        loc: {
          start: {
            line: 1,
            column: 0,
          },
          end: {
            line: 1,
            column: 22,
          },
        },
        range: [0, 22],
        callee: {
          type: "MemberExpression",
          start: 0,
          end: 11,
          loc: {
            start: {
              line: 1,
              column: 0,
            },
            end: {
              line: 1,
              column: 11,
            },
          },
          range: [0, 11],
          object: {
            type: "Identifier",
            start: 0,
            end: 7,
            loc: {
              start: {
                line: 1,
                column: 0,
              },
              end: {
                line: 1,
                column: 7,
              },
            },
            range: [0, 7],
            name: "console",
          },
          property: {
            type: "Identifier",
            start: 8,
            end: 11,
            loc: {
              start: {
                line: 1,
                column: 8,
              },
              end: {
                line: 1,
                column: 11,
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
            start: 12,
            end: 21,
            loc: {
              start: {
                line: 1,
                column: 12,
              },
              end: {
                line: 1,
                column: 21,
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
      start: 24,
      end: 33,
      loc: {
        start: {
          line: 2,
          column: 0,
        },
        end: {
          line: 2,
          column: 9,
        },
      },
      range: [24, 33],
    },
    {
      type: "ExpressionStatement",
      start: 34,
      end: 56,
      loc: {
        start: {
          line: 3,
          column: 0,
        },
        end: {
          line: 3,
          column: 22,
        },
      },
      range: [34, 56],
      expression: {
        type: "CallExpression",
        start: 34,
        end: 55,
        loc: {
          start: {
            line: 3,
            column: 0,
          },
          end: {
            line: 3,
            column: 21,
          },
        },
        range: [34, 55],
        callee: {
          type: "MemberExpression",
          start: 34,
          end: 45,
          loc: {
            start: {
              line: 3,
              column: 0,
            },
            end: {
              line: 3,
              column: 11,
            },
          },
          range: [34, 45],
          object: {
            type: "Identifier",
            start: 34,
            end: 41,
            loc: {
              start: {
                line: 3,
                column: 0,
              },
              end: {
                line: 3,
                column: 7,
              },
            },
            range: [34, 41],
            name: "console",
          },
          property: {
            type: "Identifier",
            start: 42,
            end: 45,
            loc: {
              start: {
                line: 3,
                column: 8,
              },
              end: {
                line: 3,
                column: 11,
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
            start: 46,
            end: 54,
            loc: {
              start: {
                line: 3,
                column: 12,
              },
              end: {
                line: 3,
                column: 20,
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
  sourceType: "module",
  comments: [],
  tokens: [
    {
      type: "Identifier",
      value: "console",
      start: 0,
      end: 7,
      loc: {
        start: {
          line: 1,
          column: 0,
        },
        end: {
          line: 1,
          column: 7,
        },
      },
      range: [0, 7],
    },
    {
      type: "Punctuator",
      value: ".",
      start: 7,
      end: 8,
      loc: {
        start: {
          line: 1,
          column: 7,
        },
        end: {
          line: 1,
          column: 8,
        },
      },
      range: [7, 8],
    },
    {
      type: "Identifier",
      value: "log",
      start: 8,
      end: 11,
      loc: {
        start: {
          line: 1,
          column: 8,
        },
        end: {
          line: 1,
          column: 11,
        },
      },
      range: [8, 11],
    },
    {
      type: "Punctuator",
      value: "(",
      start: 11,
      end: 12,
      loc: {
        start: {
          line: 1,
          column: 11,
        },
        end: {
          line: 1,
          column: 12,
        },
      },
      range: [11, 12],
    },
    {
      type: "String",
      value: '"hello, "',
      start: 12,
      end: 21,
      loc: {
        start: {
          line: 1,
          column: 12,
        },
        end: {
          line: 1,
          column: 21,
        },
      },
      range: [12, 21],
    },
    {
      type: "Punctuator",
      value: ")",
      start: 21,
      end: 22,
      loc: {
        start: {
          line: 1,
          column: 21,
        },
        end: {
          line: 1,
          column: 22,
        },
      },
      range: [21, 22],
    },
    {
      type: "Punctuator",
      value: ";",
      start: 22,
      end: 23,
      loc: {
        start: {
          line: 1,
          column: 22,
        },
        end: {
          line: 1,
          column: 23,
        },
      },
      range: [22, 23],
    },
    {
      type: "Keyword",
      value: "debugger",
      start: 24,
      end: 32,
      loc: {
        start: {
          line: 2,
          column: 0,
        },
        end: {
          line: 2,
          column: 8,
        },
      },
      range: [24, 32],
    },
    {
      type: "Punctuator",
      value: ";",
      start: 32,
      end: 33,
      loc: {
        start: {
          line: 2,
          column: 8,
        },
        end: {
          line: 2,
          column: 9,
        },
      },
      range: [32, 33],
    },
    {
      type: "Identifier",
      value: "console",
      start: 34,
      end: 41,
      loc: {
        start: {
          line: 3,
          column: 0,
        },
        end: {
          line: 3,
          column: 7,
        },
      },
      range: [34, 41],
    },
    {
      type: "Punctuator",
      value: ".",
      start: 41,
      end: 42,
      loc: {
        start: {
          line: 3,
          column: 7,
        },
        end: {
          line: 3,
          column: 8,
        },
      },
      range: [41, 42],
    },
    {
      type: "Identifier",
      value: "log",
      start: 42,
      end: 45,
      loc: {
        start: {
          line: 3,
          column: 8,
        },
        end: {
          line: 3,
          column: 11,
        },
      },
      range: [42, 45],
    },
    {
      type: "Punctuator",
      value: "(",
      start: 45,
      end: 46,
      loc: {
        start: {
          line: 3,
          column: 11,
        },
        end: {
          line: 3,
          column: 12,
        },
      },
      range: [45, 46],
    },
    {
      type: "String",
      value: '"world!"',
      start: 46,
      end: 54,
      loc: {
        start: {
          line: 3,
          column: 12,
        },
        end: {
          line: 3,
          column: 20,
        },
      },
      range: [46, 54],
    },
    {
      type: "Punctuator",
      value: ")",
      start: 54,
      end: 55,
      loc: {
        start: {
          line: 3,
          column: 20,
        },
        end: {
          line: 3,
          column: 21,
        },
      },
      range: [54, 55],
    },
    {
      type: "Punctuator",
      value: ";",
      start: 55,
      end: 56,
      loc: {
        start: {
          line: 3,
          column: 21,
        },
        end: {
          line: 3,
          column: 22,
        },
      },
      range: [55, 56],
    },
  ],
};
