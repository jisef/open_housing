<script>
  import CheckedInButton from '$lib/components/CheckedInButton.svelte';
  import EditButton from '$lib/components/EditButton.svelte';

  let { start, end, room, checked_in } = $props();


</script>

<style>

div span {
    padding: 0.2rem;
}
</style>

<div class="list-element">
  <div style="position: relative; top: 0px;">
    <span class="float-right"><EditButton/></span>
    <span class="float-right"><CheckedInButton checkedIn="{checked_in}"/></span>
  </div>
  <p>start {start}</p>
  <p>end {end}</p>
  <p>room {room}</p>

</div>

