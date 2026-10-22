import js from '@eslint/js';
import vue from 'eslint-plugin-vue';
import ts from 'typescript-eslint';
import globals from 'globals';

export default ts.config(
    js.configs.recommended,
    ...ts.configs.recommended,
    ...vue.configs['flat/essential'],
    {
        files: ['src/**/*.{ts,vue}'],
        languageOptions: {
            globals: globals.browser,
            parserOptions: { parser: ts.parser },
        },
        rules: {
            '@typescript-eslint/no-explicit-any': 'off', // Existing metrics are dynamically keyed.
        },
    },
);
