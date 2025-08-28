#!/bin/bash

echo "Testing Python Plugin System for Rest Reminder"
echo "================================================="

echo ""
echo "Plugin directory contents:"
cd ..
ls -la plugins/

echo ""
echo "Python plugins found:"
find plugins -name "*.py" -exec basename {} \;

echo ""
echo "Starting Rest Reminder with plugin system..."
echo "   (Will monitor for 'NonExistentApp' - should show plugin initialization)"
echo "   Press Ctrl+C to stop after seeing the plugin output"
echo ""

./target/debug/rest-reminder rest -t 10 -a NonExistentApp