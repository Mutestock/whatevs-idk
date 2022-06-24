export class PostgresOptions {
    host: string;
    usr: string;
    pwd: string | null;
    port: number;
    db_name: string;

    constructor(
        host: string = "",
        usr: string = "",
        pwd: string | null = null,
        port: number = 5432,
        db_name: string = "") {

        this.host = host;
        this.usr = usr;
        this.pwd = pwd;
        this.port = port;
        this.db_name = db_name;
    }
}

export class MongoDBOptions {
    host: string;
    user: string;
    pwd: string | null;
    port: number;

    constructor(host: string, user: string, pwd: string | null, port: number) {
        this.host = host;
        this.user = user;
        this.pwd = pwd;
        this.port = port;
    }
}