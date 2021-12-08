use std::ops::Neg;

use super::MissionRpc;
use stubs::custom::v0::custom_service_server::CustomService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl CustomService for MissionRpc {
    async fn request_mission_assignment(
        &self,
        request: Request<custom::v0::RequestMissionAssignmentRequest>,
    ) -> Result<Response<custom::v0::RequestMissionAssignmentResponse>, Status> {
        self.notification("requestMissionAssignment", request)
            .await?;
        Ok(Response::new(
            custom::v0::RequestMissionAssignmentResponse {},
        ))
    }

    async fn join_mission(
        &self,
        request: Request<custom::v0::JoinMissionRequest>,
    ) -> Result<Response<custom::v0::JoinMissionResponse>, Status> {
        self.notification("joinMission", request).await?;
        Ok(Response::new(custom::v0::JoinMissionResponse {}))
    }

    async fn eval(
        &self,
        request: Request<custom::v0::EvalRequest>,
    ) -> Result<Response<custom::v0::EvalResponse>, Status> {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }

        let json: serde_json::Value = self.request("missionEval", request).await?;
        let json = serde_json::to_string(&json).map_err(|err| {
            Status::internal(format!("failed to deserialize eval result: {}", err))
        })?;
        Ok(Response::new(custom::v0::EvalResponse { json }))
    }

    async fn get_magnetic_declination(
        &self,
        request: Request<custom::v0::GetMagneticDeclinationRequest>,
    ) -> Result<Response<custom::v0::GetMagneticDeclinationResponse>, Status> {
        let position = request.into_inner();

        // As only the date is relevant, and a difference of some days don't really matter, it is
        // fine to just use the scenario's start time, especially since it is cached and thus
        // prevents unnecessary roundtrips to the MSE.
        let date = self.get_scenario_start_time().await?.date();
        let declination = igrf::declination(position.lat, position.lon, position.alt as u32, date)
            .map(|f| f.d)
            .or_else(|err| match err {
                igrf::Error::DateOutOfRange(f) => Ok(f.d),
                err => Err(Status::internal(format!(
                    "failed to estimate magnetic declination: {}",
                    err
                ))),
            })?;

        // reduce precision to two decimal places
        let declination = ((declination * 100.0).round() / 100.0).neg();

        Ok(Response::new(custom::v0::GetMagneticDeclinationResponse {
            declination,
        }))
    }

    async fn get_tickets(
        &self,
        request: Request<custom::v0::GetTicketsRequest>,
    ) -> Result<Response<custom::v0::GetTicketsResponse>, Status> {
        let res: custom::v0::GetTicketsResponse = self.request("getTickets", request).await?;
        Ok(Response::new(res))
    }

    async fn initialize_tickets(
        &self,
        request: Request<custom::v0::InitializeTicketsRequest>,
    ) -> Result<Response<custom::v0::InitializeTicketsResponse>, Status> {
        self.request("initializeTickets", request).await?;
        Ok(Response::new(custom::v0::InitializeTicketsResponse {}))
    }

    async fn initialize_factory_objectives(
        &self,
        request: Request<custom::v0::InitializeFactoryObjectivesRequest>,
    ) -> Result<Response<custom::v0::InitializeFactoryObjectivesResponse>, Status> {
        self.request("initializeFactoryObjectives", request).await?;
        Ok(Response::new(custom::v0::InitializeFactoryObjectivesResponse {}))
    }

    async fn initialize_capture_point(
        &self,
        request: Request<custom::v0::InitializeCapturePointRequest>,
    ) -> Result<Response<custom::v0::InitializeCapturePointResponse>, Status> {
        self.request("initializeCapturePoint", request).await?;
        Ok(Response::new(custom::v0::InitializeCapturePointResponse {}))
    }

    async fn initialize_player_points(
        &self,
        request: Request<custom::v0::InitializePlayerPointsRequest>,
    ) -> Result<Response<custom::v0::InitializePlayerPointsResponse>, Status> {
        self.request("initializePlayerPoints", request).await?;
        Ok(Response::new(custom::v0::InitializePlayerPointsResponse {}))
    }

    async fn get_player_points(
        &self,
        request: Request<custom::v0::GetPlayerPointsRequest>,
    ) -> Result<Response<custom::v0::GetPlayerPointsResponse>, Status> {
        let res: custom::v0::GetPlayerPointsResponse = self.request("getPlayerPoints", request).await?;
        Ok(Response::new(res))
    }

    async fn credit_player_points(
        &self,
        request: Request<custom::v0::CreditPlayerPointsRequest>,
    ) -> Result<Response<custom::v0::CreditPlayerPointsResponse>, Status> {
        self.request("creditPlayerPoints", request).await?;
        Ok(Response::new(custom::v0::CreditPlayerPointsResponse {}))
    }

    async fn on_zone_captured(
        &self,
        request: Request<custom::v0::OnZoneCapturedRequest>,
    ) -> Result<Response<custom::v0::OnZoneCapturedResponse>, Status> {
        self.request("onZoneCaptured", request).await?;
        Ok(Response::new(custom::v0::OnZoneCapturedResponse {}))
    }

    async fn is_zone_empty(
        &self,
        request: Request<custom::v0::IsZoneEmptyRequest>,
    ) -> Result<Response<custom::v0::IsZoneEmptyResponse>, Status> {
        let res: custom::v0::IsZoneEmptyResponse = self.request("isZoneEmpty", request).await?;
        Ok(Response::new(res))
    }

    async fn get_zone_statuses(
        &self,
        request: Request<custom::v0::GetZoneStatusesRequest>,
    ) -> Result<Response<custom::v0::GetZoneStatusesResponse>, Status> {
        let res: custom::v0::GetZoneStatusesResponse = self.request("getZoneStatuses", request).await?;
        Ok(Response::new(res))
    }

    async fn get_factory_state(
        &self,
        request: Request<custom::v0::GetFactoryStateRequest>,
    ) -> Result<Response<custom::v0::GetFactoryStateResponse>, Status> {
        let res: custom::v0::GetFactoryStateResponse = self.request("getFactoryState", request).await?;
        Ok(Response::new(res))
    }

    async fn send_message_from_hq(
        &self,
        request: Request<custom::v0::SendMessageFromHqRequest>,
    ) -> Result<Response<custom::v0::SendMessageFromHqResponse>, Status> {
        self.request("sendMessageFromHQ", request).await?;
        Ok(Response::new(custom::v0::SendMessageFromHqResponse {}))
    }
}
