function add(lhs, rhs) {
    return lhs + rhs
};
var x = function add2(lhs, rhs) {
    return lhs + rhs
};
var lambda_owner = (function (a, b, c) {
    return a + b + c
})

console.log(add(2,4))
console.log(x(1,2))
console.log(lambda_owner(1,2,3))