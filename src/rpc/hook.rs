use std::pin::Pin;

use super::HookRpc;
use crate::shutdown::AbortableStream;
use futures_util::{Stream, TryStreamExt};
use stubs::hook::v0::hook_service_server::HookService;
use stubs::*;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl HookService for HookRpc {
    type StreamChatMessagesStream = Pin<
        Box<
            dyn Stream<Item = Result<hook::v0::StreamChatMessagesResponse, tonic::Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    type StreamPlayerConnectEventsStream = Pin<
        Box<
            dyn Stream<Item = Result<hook::v0::StreamPlayerConnectEventsResponse, tonic::Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn get_mission_name(
        &self,
        request: Request<hook::v0::GetMissionNameRequest>,
    ) -> Result<Response<hook::v0::GetMissionNameResponse>, Status> {
        let res: hook::v0::GetMissionNameResponse = self.request("getMissionName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_info(
        &self,
        request: Request<hook::v0::GetPlayerInfoRequest>,
    ) -> Result<Response<hook::v0::GetPlayerInfoResponse>, Status> {
        let res: hook::v0::GetPlayerInfoResponse = self.request("getPlayerInfo", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_info_by_player_name(
        &self,
        request: Request<hook::v0::GetPlayerInfoByPlayerNameRequest>,
    ) -> Result<Response<hook::v0::GetPlayerInfoByPlayerNameResponse>, Status> {
        let res: hook::v0::GetPlayerInfoByPlayerNameResponse = self.request("getPlayerInfoByPlayerName", request).await?;
        Ok(Response::new(res))
    }

    async fn stream_chat_messages(
        &self,
        _request: Request<hook::v0::StreamChatMessagesRequest>,
    ) -> Result<Response<Self::StreamChatMessagesStream>, Status> {
        let rx = BroadcastStream::new(self.chat.subscribe());
        let stream = AbortableStream::new(
            self.shutdown_signal.signal(),
            rx.map_err(|err| Status::unknown(err.to_string())),
        );
        Ok(Response::new(Box::pin(stream)))
    }

    async fn stream_player_connect_events(
        &self,
        _request: Request<hook::v0::StreamPlayerConnectEventsRequest>,
    ) -> Result<Response<Self::StreamPlayerConnectEventsStream>, Status> {
        log::info!("hook::stream_player_connect_events");
        let rx = BroadcastStream::new(self.connect_event.subscribe());
        let stream = AbortableStream::new(
            self.shutdown_signal.signal(),
            rx.map_err(|err| Status::unknown(err.to_string())),
        );
        Ok(Response::new(Box::pin(stream)))
    }

    async fn eval(
        &self,
        request: Request<hook::v0::EvalRequest>,
    ) -> Result<Response<hook::v0::EvalResponse>, Status> {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }

        let json: serde_json::Value = self.request("hookEval", request).await?;
        let json = serde_json::to_string(&json).map_err(|err| {
            Status::internal(format!("failed to deserialize eval result: {}", err))
        })?;
        Ok(Response::new(hook::v0::EvalResponse { json }))
    }
}
