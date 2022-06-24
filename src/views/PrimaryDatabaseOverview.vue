<script setup lang="ts">
import { useDatabaseInteractionStore } from "../stores/databaseInteractionStore";

const databaseInteractionStore = useDatabaseInteractionStore();
</script>


<template>
    <div>
        <p>Host: {{ databaseInteractionStore.postgres_options.host }}</p>
        <p>Port: {{ databaseInteractionStore.postgres_options.port }}</p>
        <p>Usr: {{ databaseInteractionStore.postgres_options.usr }}</p>
        <p password>Pwd: {{ databaseInteractionStore.postgres_options.pwd }}</p>
        <p>DB Name: {{ databaseInteractionStore.postgres_options.db_name }}</p>
    </div>
    <div>
        <div>
            <label for="host-input">Host</label>
            <input v-model="databaseInteractionStore.postgres_options.host" id="host-input" />
        </div>
        <div>
            <label for="port-input">Port</label>
            <input v-model="databaseInteractionStore.postgres_options.port" />
        </div>
        <div>
            <label for="usr-input">User</label>
            <input v-model="databaseInteractionStore.postgres_options.usr" />
        </div>
        <div>
            <label for="pwd-input">Password</label>
            <input type="password" v-model="databaseInteractionStore.postgres_options.pwd" />
        </div>

        <div>
            <label for="db-name-input">db_name</label>
            <input v-model="databaseInteractionStore.postgres_options.db_name" />
        </div>
        <button @click='databaseInteractionStore.select_all("people")'>Submit</button>
        <p>Res= {{databaseInteractionStore.select_all_response}}</p>
        <!--<p v-if="databaseInteractionStore.select_all_response!==''">Res={{databaseInteractionStore.select_all_response}}</p>
        <p v-else>No result from the database at this time =(</p>-->
    </div>
</template>

