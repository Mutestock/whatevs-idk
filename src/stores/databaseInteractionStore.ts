import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";
import { PostgresOptions } from "../utils/connectionStructures";

export const useDatabaseInteractionStore = defineStore("databaseInteractionStore", {
    state: () => {
        return { postgres_options: new PostgresOptions(), select_all_response: "flerp" }
    },
    actions: {
        select_all(table_name: string) {
            //this.select_all_response = "Making query on: " + table_name;
            let invocation: Promise<string> = invoke("invoke_postgres_select_all_query", {
                table_name: table_name,
                //pg_options: this.postgres_options
            }) as Promise<string>;
            invocation.then((response) => this.select_all_response = response);

            //this.select_all_response = "Failed without response =(";
        },
        whatever_man() {
            const invocation: Promise<string> = invoke('invoke_whatever', { table_name: 'table_name' });
            invocation.then((response) => this.select_all_response = response);
        },
        gen_pass(cake: number) {
            const invocation: Promise<string> = invoke('generate_password', {
                length: cake
            }) as Promise<string>;
            invocation.then((response) => this.select_all_response = response);
            //this.select_all_response = cake;
        }
    },
});