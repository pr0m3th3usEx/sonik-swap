-- CreateTable
CREATE TABLE "User" (
    "id" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "MusicAccountProvider" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "color" TEXT NOT NULL,
    "base_url" TEXT NOT NULL,
    "token_url" TEXT NOT NULL,
    "authorizations_needed" TEXT[],
    "configuration" JSONB NOT NULL,

    CONSTRAINT "MusicAccountProvider_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "MusicAccount" (
    "user_id" TEXT NOT NULL,
    "map_id" TEXT NOT NULL,
    "accessTokenSecretKey" TEXT NOT NULL,
    "refreshTokenSecretKey" TEXT NOT NULL,

    CONSTRAINT "MusicAccount_pkey" PRIMARY KEY ("user_id","map_id")
);

-- CreateIndex
CREATE UNIQUE INDEX "User_username_key" ON "User"("username");

-- CreateIndex
CREATE UNIQUE INDEX "User_email_key" ON "User"("email");

-- AddForeignKey
ALTER TABLE "MusicAccount" ADD CONSTRAINT "MusicAccount_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MusicAccount" ADD CONSTRAINT "MusicAccount_map_id_fkey" FOREIGN KEY ("map_id") REFERENCES "MusicAccountProvider"("id") ON DELETE CASCADE ON UPDATE CASCADE;
