cargo build

DEBUG_PATH=./target/debug/unicode-formatter

# test delemeter
echo "alpha ;alpha" | $DEBUG_PATH

# test repitition
echo ";alpha ;alpha" | $DEBUG_PATH 

# test using different identifiers
echo ";alpha ;zeta" | $DEBUG_PATH

# test case sensitivity
echo ";Alpha ;alpha" | $DEBUG_PATH

# test load config
echo ";Alpha ;alpha" | $DEBUG_PATH --config ./conf.toml

# test special delemeter
echo "#Dalpha" | $DEBUG_PATH --delemeter "#D"

# test no delimiter
echo ";not_one ;alpha" | $DEBUG_PATH
