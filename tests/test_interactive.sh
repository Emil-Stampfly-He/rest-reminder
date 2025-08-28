#!/bin/bash

echo "Testing Rest Reminder Interactive Mode..."
echo
cd ..

# Test 1: Interactive mode with help command
echo "=== Test 1: Help command ==="
echo -e "help\nexit" | ./target/release/rest-reminder
echo

# Test 2: Command line mode still works
echo "=== Test 2: Command line mode ==="
./target/release/rest-reminder rest --help
echo

# Test 3: App bundle interactive mode
echo "=== Test 3: App bundle interactive mode ==="
echo -e "help\nexit" | ./target/release/RestReminder.app/Contents/MacOS/RestReminder
echo

echo "Testing completed!"
