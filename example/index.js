var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import init, { add } from './pkg/rust_fel_example.js';
function run() {
    return __awaiter(this, void 0, void 0, function* () {
        yield init();
        const result = add(1, 2);
        console.log(`1 + 2 = ${result}`);
        if (result !== 3)
            throw new Error("wasm addition doesn't work!");
    });
}
run();
//# sourceMappingURL=index.js.map