<script>
	import { createEventDispatcher } from 'svelte';
  import { get_current_component } from 'svelte/internal';

  const component = get_current_component();
  const svelteDispatch = createEventDispatcher();
  
  const dispatch = (name, detail) => {
    svelteDispatch(name, detail);
    component.dispatchEvent && component.dispatchEvent(new CustomEvent(name, { detail }));
  };

  function handleBack(){
    dispatch("message");
    console.log("hello");
  }
</script>

<div id="setup">
  <input type="button" value="Back" on:click={handleBack}>
  <h1>ReceiveFile</h1>
</div>

<style>
  #setup{
    height: 100%;
    width: 100%;
    color: var(--primaryColor);
  }
</style>