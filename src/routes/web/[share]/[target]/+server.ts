import type { RequestHandler } from './$types';
import { init } from "$lib/sync";
import { redirect } from '@sveltejs/kit';

export const GET = (({params}) => {
  init(params.share, params.target);

  return redirect(300, '/');
  
}) satisfies RequestHandler;
