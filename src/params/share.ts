import type { ParamMatcher } from '@sveltejs/kit';

export const match: ParamMatcher = (param) => {
    return /\+.{1,15}\.b[a-z]{52}/.test(param);
};