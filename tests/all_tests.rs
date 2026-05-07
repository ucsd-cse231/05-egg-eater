mod infra;

// Your tests go here!
success_tests! {
    {
        name: simple_examples,
        file: "simple_examples.snek",
        expected: "(vec 1 2 3)\n(vec false true)\n(vec 0 true (vec 4 3 (vec 2 1)) (vec 11 22 33) true)\ntrue\n2\n3\n5\n3\n(vec 100 2 3)\n2\n(vec 9 8 7 6)\n(vec 400 3 (vec 2 1))",
    },
    {
        name: points,
        file: "points.snek",
        expected: "(vec 4 5)\n(vec -4 2)\n(vec 0 7)\n(vec 8 10)\n(vec 2 0)\n(vec 3 -9)\n(vec 5 -9)\n(vec 5 -9)\n(vec 7 -4)\n(vec 5 -2)",
    },
    {
        name: vec,
        file: "vec.snek",
        expected: "(vec 1 2 3 4 5 6 7 8 9 10)",
    },
    {
        name: nested_vec,
        file: "nested_vec.snek",
        expected: "(vec 1 2 3 (vec false true false (vec -1 -3 -9) 33) true (vec 11 22 33 -44) (vec 19 17 15 false))",
    },

    // DIAMONDBACK TESTS BELOW THIS POINT
    // COBRA TESTS BELOW THIS POINT
}

runtime_error_tests! {
    {
        name: error_tag,
        file: "error-tag.snek",
        expected: "invalid argument",
    },
    {
        name: error_bounds,
        file: "error-bounds.snek",
        expected: "index out of bounds",
    },
    {
        name: error3,
        file: "error3.snek",
        expected: "out of heap space",
    },

    // COBRA TESTS BELOW THIS POINT

}

static_error_tests! {
    // DIAMONDBACK TESTS BELOW THIS POINT
    // COBRA TESTS BELOW THIS POINT
}
