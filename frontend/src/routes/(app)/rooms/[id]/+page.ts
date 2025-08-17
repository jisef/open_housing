import type { IRoom } from '$lib/types/Room';
import type { PageLoad } from '../../../../../.svelte-kit/types/src/routes';
import type { APIResponse } from '$lib/types/APIResponse';

export const load: PageLoad = async ({ fetch, params }) => {
  // load room
  try {
    const res: APIResponse = await fetch(`/api/rooms/${params.id}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(x => x.json());

    return {
      room: res.data as IRoom
    };

  } catch (err) {
    console.error(err);
  }
};
