local amble = require('amble')

amble.init()

vim.g.mapleader = " "

vim.api.nvim_set_keymap('n', '<leader>q', ':q!<CR>', {})
vim.api.nvim_set_keymap('n', '<leader>aw', ':AmbleWrite<CR>', {})
vim.api.nvim_set_keymap('n', '<leader>al', ':AmbleList<CR>', {})
