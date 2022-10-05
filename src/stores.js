import { writable } from 'svelte/store';

export const fileSet = writable([]);

export const config = writable({
    "settings" : [],
    "initState" : true
});