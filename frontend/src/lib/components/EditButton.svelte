<style>
    .edit-button {
        width: 32px;
        height: 31px;
    }
</style>

<script lang="ts">
  import Popup from '$lib/components/popup/Popup.svelte';
  import type { SvelteComponent } from 'svelte';
  type ComponentProps = Record<string, unknown>;
  type ComponentType = ConstructorOfATypedSvelteComponent | SvelteComponent;

  let { onClose, component, componentProps }: { onClose: unknown, component: ComponentType , componentProps: ComponentProps  } = $props();
  let isOpen = $state(false)

  function onclick() {
    isOpen = !isOpen;
  }
</script>

<svg {onclick} class="text-gray-800 dark:text-white edit-button icon-button" aria-hidden="true"
     xmlns="http://www.w3.org/2000/svg" width="50" height="24" fill="none" viewBox="0 0 24 24">
  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
        d="m14.304 4.844 2.852 2.852M7 7H4a1 1 0 0 0-1 1v10a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1v-4.5m2.409-9.91a2.017 2.017 0 0 1 0 2.853l-6.844 6.844L8 14l.713-3.565 6.844-6.844a2.015 2.015 0 0 1 2.852 0Z" />
</svg>

<Popup {isOpen} component={component} componentProps={componentProps}   />


