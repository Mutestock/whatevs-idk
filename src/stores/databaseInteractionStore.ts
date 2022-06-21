import { defineStore } from "pinia";

export const databaseInteractionStore = defineStore("databaseInteractionStore", {
    state: () => {
        return {}
    },
    actions: {
        stuff() {
            console.log("cake")
        }
    },
});