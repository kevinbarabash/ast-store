import { foobar as _foobar } from "./rules/foobar.js";
import { foo } from "./foo.js";

export const foobar = foo + _foobar;
