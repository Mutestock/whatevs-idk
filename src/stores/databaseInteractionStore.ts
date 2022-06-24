import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";
import { PostgresOptions } from "../utils/connectionStructures";

export const databaseInteractionStore = defineStore("databaseInteractionStore", {
    state: () => {
        return { postgres_options: new PostgresOptions()}
    },
    actions: {
        select_all(table_name: string): string {
            const invocation: Promise<string> = invoke("invoke_postgres_select_all_query",{
                table_name:table_name,
                pg_options: this.postgres_options
            });
            let res: string = "";
            invocation.then(response => res = response);
            return res;
        }
    },
});