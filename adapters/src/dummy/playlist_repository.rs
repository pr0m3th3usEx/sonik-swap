use std::collections::{HashMap, HashSet};

use chrono::DateTime;
use snk_core::{
    contracts::repositories::playlist_repository::{PlaylistRepository, PlaylistRepositoryResult},
    entities::{album::Album, artist::Artist, playlist::Playlist, track::TrackWithAlbumAndArtists},
    value_objects::{
        image_cover::ImageCover, playlist_id::PlaylistId, product_id::ProductId,
        provider::provider_id::ProviderId,
    },
};
use url::Url;

pub struct DummyPlaylistRepository {}

impl PlaylistRepository for DummyPlaylistRepository {
    async fn get(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Option<Playlist>> {
        Ok(Some(Playlist::new(
            id.clone(),
            String::from("Emo"),
            HashSet::new(),
            String::from("me"),
            5,
            "https://www.deezer.com/us/playlist/9701198282"
                .parse()
                .unwrap(),
        )))
    }

    async fn get_all(&self) -> PlaylistRepositoryResult<Vec<Playlist>> {
        Ok(vec![Playlist::new(
            PlaylistId::LikedSongs,
            String::from("Emo"),
            HashSet::new(),
            String::from("me"),
            5,
            "https://www.deezer.com/us/playlist/9701198282"
                .parse()
                .unwrap(),
        )])
    }

    async fn create(&self, name: &str) -> PlaylistRepositoryResult<Playlist> {
        Ok(Playlist::new(
            PlaylistId::Owned(String::from("deezer_playlist_id")),
            name.to_string(),
            HashSet::new(),
            String::from("me"),
            5,
            "https://www.deezer.com/us/playlist/9701198282"
                .parse()
                .unwrap(),
        ))
    }

    async fn delete(&self, id: &PlaylistId) -> PlaylistRepositoryResult<Playlist> {
        Ok(Playlist::new(
            id.clone(),
            String::from("Emo"),
            HashSet::new(),
            String::from("me"),
            5,
            "https://www.deezer.com/us/playlist/9701198282"
                .parse()
                .unwrap(),
        ))
    }

    async fn add_tracks(
        &self,
        _playlist_id: &PlaylistId,
        _ids: &[String],
    ) -> PlaylistRepositoryResult<()> {
        Ok(())
    }

    async fn delete_tracks(
        &self,
        _playlist_id: &PlaylistId,
        _ids: &[String],
    ) -> PlaylistRepositoryResult<()> {
        Ok(())
    }

    async fn get_tracks(
        &self,
        _playlist_id: &PlaylistId,
    ) -> PlaylistRepositoryResult<Vec<TrackWithAlbumAndArtists>> {
        Ok(vec![TrackWithAlbumAndArtists::new(
            HashSet::from_iter([ProductId::Provider((ProviderId::new(String::from("deezer")), String::from("deezer_track_id")))]),
            String::from("Night Like This"),
            120000,
            HashMap::from_iter([(
                ProviderId::new(String::from("deezer")),
                Url::parse("https://deezer.page.link/uivRrAmu77R4k27v5").unwrap(),
            )]),
            Album::new(
              HashSet::from_iter([(ProductId::Provider((ProviderId::new(String::from("deezer")), String::from("deezer_album_id"))))]),
              String::from("Nights Like This (feat. Ty Dolla $ign)"),
              DateTime::parse_from_rfc3339("2020-04-12T22:10:57+02:00")
              .unwrap()
              .into(),
              HashSet::from_iter([
                  ImageCover::Sm(Url::parse("https://cdn-images.dzcdn.net/images/cover/38f53c7ad2ef060d90f500a597e0f2f5/500x500-000000-80-0-0.jpg").unwrap())
              ]),
              HashMap::from_iter([
                  (ProviderId::new(String::from("deezer")), Url::parse("https://deezer.page.link/vF5Jvr4NnHQPhgJR6").unwrap())
              ])
            ),
            vec![
                Artist::new(
                    HashMap::from_iter([
                        (ProviderId::new(String::from("deezer")), String::from("deezer_artist_id"))
                    ]),
                    String::from("Khelani"),
                    HashSet::from_iter([
                        ImageCover::Sm(Url::parse("https://cdn-images.dzcdn.net/images/artist/2bf1fa3d1cc1716f784dadf112d16d9e/500x500-000000-80-0-0.jpg").unwrap())
                    ]),
                    HashMap::from_iter([
                        (ProviderId::new(String::from("deezer")), Url::parse("https://www.deezer.com/us/artist/5603027").unwrap())
                    ])
                )
            ])
          ])
    }
}
