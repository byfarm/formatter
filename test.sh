DEBUG_PATH=./target/debug/unicode-formatter

# test delemeter
echo "alpha #Ualpha" | $DEBUG_PATH

# test repitition
echo "#Ualpha #Ualpha" | $DEBUG_PATH 

# test using different identifiers
echo "#Ualpha #Uzeta" | $DEBUG_PATH

# test case sensitivity
echo "#UAlpha #Ualpha" | $DEBUG_PATH

# test load config
echo "#UAlpha #Ualpha" | $DEBUG_PATH --config ./conf.toml

# test special delemeter
echo "#Dalpha" | $DEBUG_PATH --delemeter "#D"
