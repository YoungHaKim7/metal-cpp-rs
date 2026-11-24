# Check all 102 examples
check-all:
    cargo check --examples

# Build all 102 examples
build-all:
    cargo build --examples

# Run Clippy on all examples
clippy-all:
    cargo clippy --examples -- -D warnings

# Check one example
check example:
    cargo check --example {{example}}

# Build one example
build example:
    cargo build --example {{example}}

# Run one example
run example:
    cargo run --example {{example}}

# Check examples individually and report failures
check-each:
    #!/usr/bin/env bash
    set +e

    examples=(
        $(cargo metadata --no-deps --format-version 1 |
          python3 -c 'import json,sys; d=json.load(sys.stdin); print(" ".join(sorted({t["name"] for p in d["packages"] for t in p["targets"] if "example" in t["kind"]})))')
    )

    total=0
    failed=0

    for example in "${examples[@]}"; do
        total=$((total + 1))
        echo ""
        echo "[$total/${#examples[@]}] Checking $example"

        if cargo check --example "$example"; then
            echo "PASS: $example"
        else
            echo "FAIL: $example"
            failed=$((failed + 1))
        fi
    done

    echo ""
    echo "================================"
    echo "Total:  $total"
    echo "Passed: $((total - failed))"
    echo "Failed: $failed"
    echo "================================"

    test "$failed" -eq 0
