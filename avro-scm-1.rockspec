package = 'avro'
version = 'scm-1'
source = {
    url = 'git://github.com/oleggator/avro-lua.git',
    branch = 'master',
}
description = {
    summary = "",
    homepage = '',
    license = 'MIT',
}
dependencies = {
    'lua >= 5.1',
}
external_dependencies = {}

build = {
    type = 'make',

    variables = {
        version = 'scm-1',
        TARANTOOL_DIR = '$(TARANTOOL_DIR)',
        TARANTOOL_INSTALL_LIBDIR = '$(LIBDIR)',
        TARANTOOL_INSTALL_LUADIR = '$(LUADIR)',
        CC = 'gcc',
    }
}

-- vim: syntax=lua
