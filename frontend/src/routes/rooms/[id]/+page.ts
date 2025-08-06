import type { IRoom } from '$lib/types/Room';
import type { PageLoad } from './$types';
import type { Response } from '$lib/types/Response';

export const load: PageLoad = async ({ fetch, params }) => {
  // load room
  try {
    const res: Response = await fetch(`/api/rooms/${params.id}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    }).then(x => x.json());
    const room: IRoom = res.data;
    console.log(room);

    return {
      room: res.data
    };

  } catch (err) {
    console.error(err);
  }
};
