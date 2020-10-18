function main() {
    let blk_test = (/* block */ function(){
        let a = 5;
        return a;
    })()
    let collapsed_lambda = function(){
        return 6;
    }
    console.assert(blk_test == 5);
    console.assert(typeof collapsed_lambda == "function");
    console.assert(collapsed_lambda() == 6);
}
main();