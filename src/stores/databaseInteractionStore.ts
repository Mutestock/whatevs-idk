import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";
import { PostgresOptions } from "../utils/connectionStructures";

class Flem {
    name: string;
    flerp: string | null;

    constructor(name: string, flerp: string | null){
        this.name=name;
        this.flerp = flerp;
    }
}

class PgOptions{
    host: string;
    
    constructor(host: string){
        this.host=host;
    }
}


export const useDatabaseInteractionStore = defineStore("databaseInteractionStore", {
    state: () => {
        return { postgres_options: new PostgresOptions("localhost"), select_all_response: "flerp" }
    },
    actions: {
        select_all(table_name: string) {
            const flem: Flem = new Flem("cake", "booop");
            const options: PgOptions = new PgOptions("localhost");
            const invocation: Promise<string> = invoke('generate_password', {
                length: table_name,
                options: options
                //pg_options: this.postgres_options
            }) as Promise<string>;
            invocation.then((response) => this.select_all_response = response);

            //this.select_all_response = "Failed without response =(";
        },
        whatever_man() {
            const invocation: Promise<string> = invoke('invoke_whatever', { table_name: 'aaaah' });
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