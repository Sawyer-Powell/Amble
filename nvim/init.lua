local amble = require('amble')

amble.init()

vim.g.mapleader = " "

vim.api.nvim_set_keymap('n', '<leader>q', ':q!<CR>', {})
vim.api.nvim_set_keymap('n', '<leader>al', ':AmbleList<CR>', {})
vim.api.nvim_set_keymap('n', '<leader>an', ':AmbleNew<CR>', {})
