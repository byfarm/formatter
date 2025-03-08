repo_root := parent_directory(justfile())

default:
    @just --list

update:
    sh update_binary.sh

test:
    sh testing/test.sh

