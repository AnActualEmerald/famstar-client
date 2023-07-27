import type { ParamMatcher } from '@sveltejs/kit';

export const match: ParamMatcher = (param) => {
    try {
        new URL(decodeURIComponent(param));
        return true;
    } catch {
        return false;
    }
};