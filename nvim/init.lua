vim.o.runtimepath = vim.o.runtimepath .. ",./nvim/third-party/telescope.nvim"
vim.o.runtimepath = vim.o.runtimepath .. ",./nvim/third-party/plenary.nvim"

local amble = require('amble')
amble.init()

vim.g.mapleader = " "

vim.api.nvim_set_keymap('n', '<leader>q', ':q!<CR>', {})
vim.api.nvim_set_keymap('n', '<leader>al', ':AmbleList<CR>', {})
vim.api.nvim_set_keymap('n', '<leader>an', ':AmbleNew<CR>', {})

local builtin = require('telescope.builtin')

vim.keymap.set('n', '<leader>ff', builtin.find_files, { desc = 'Telescope find files' })
vim.keymap.set('n', '<leader>fg', builtin.live_grep, { desc = 'Telescope live grep' })
vim.keymap.set('n', '<leader>fb', builtin.buffers, { desc = 'Telescope buffers' })
vim.keymap.set('n', '<leader>fh', builtin.help_tags, { desc = 'Telescope help tags' })
