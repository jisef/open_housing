<script lang="ts">
  import type { SvelteComponent } from 'svelte';

  type ComponentProps = Record<string, unknown>;
  type ComponentType = ConstructorOfATypedSvelteComponent | SvelteComponent;

  let { isOpen, component, componentProps }: { isOpen: boolean, component: ComponentType, componentProps: ComponentProps } = $props();
</script>

{#if isOpen}
  <div class="popup-overlay">
    <div class="popup-content" on:click|stopPropagation>
      <svelte:component this={component} {componentProps}  />
    </div>
  </div>
{/if}

<style>
    .popup-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .popup-content {
        background: white;
        padding: 2rem;
        border-radius: 8px;
        max-width: 90%;
        max-height: 90vh;
        overflow: auto;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
    }
</style>