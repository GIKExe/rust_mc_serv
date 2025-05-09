use std::{
	sync::Arc,
	time::{SystemTime, UNIX_EPOCH},
};

use rust_mc_proto::{DataReader, DataWriter, Packet};

use crate::{
	ServerError,
	data::{ReadWriteNBT, component::TextComponent},
	player::context::ClientContext,
	protocol::packet_id::{clientbound, serverbound},
};

pub fn send_game_event(
	client: Arc<ClientContext>,
	event: u8,
	value: f32,
) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::GAME_EVENT);

	packet.write_byte(event)?;
	packet.write_float(value)?;

	client.write_packet(&packet)
}

pub fn send_entity_event(
	client: Arc<ClientContext>,
	entity_id: i32,
	status: u8,
) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::ENTITY_EVENT);

	packet.write_int(entity_id)?;
	packet.write_byte(status)?;

	client.write_packet(&packet)
}

pub fn send_entity_animation(
	receiver: Arc<ClientContext>,
	entity_id: i32,
	animation: u8,
) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::ENTITY_ANIMATION);

	packet.write_varint(entity_id)?;
	packet.write_byte(animation)?;

	receiver.write_packet(&packet)?;

	Ok(())
}

pub fn play_global_sound(
	receiver: Arc<ClientContext>,
	sound: String,
	volume: f32,
	pitch: f32,
	category: i32,
) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::ENTITY_SOUND_EFFECT);

	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_micros() as i64;

	packet.write_varint(0)?;
	packet.write_string(&sound)?;
	packet.write_boolean(false)?; // is fixed range
	// packet.write_float(0.0)?; // fixed range
	packet.write_varint(receiver.entity_info().unwrap().entity_id)?;
	packet.write_varint(category)?; // sound category (0 - master)
	packet.write_float(volume)?; // volume
	packet.write_float(pitch)?; // pitch
	packet.write_long(timestamp)?; // seed

	receiver.write_packet(&packet)?;

	Ok(())
}

pub fn sync_player_pos(
	client: Arc<ClientContext>,
	x: f64,
	y: f64,
	z: f64,
	vel_x: f64,
	vel_y: f64,
	vel_z: f64,
	yaw: f32,
	pitch: f32,
	flags: i32,
) -> Result<(), ServerError> {
	let timestamp = (SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis()
		& 0xFFFFFFFF) as i32;

	let mut packet = Packet::empty(clientbound::play::SYNCHRONIZE_PLAYER_POSITION);

	packet.write_varint(timestamp)?;
	packet.write_double(x)?;
	packet.write_double(y)?;
	packet.write_double(z)?;
	packet.write_double(vel_x)?;
	packet.write_double(vel_y)?;
	packet.write_double(vel_z)?;
	packet.write_float(yaw)?;
	packet.write_float(pitch)?;
	packet.write_int(flags)?;

	client.write_packet(&packet)?;

	Ok(())
}

pub fn set_center_chunk(client: Arc<ClientContext>, x: i32, z: i32) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::SET_CENTER_CHUNK);

	packet.write_varint(x)?;
	packet.write_varint(z)?;

	client.write_packet(&packet)
}

pub fn send_keep_alive(client: Arc<ClientContext>) -> Result<(), ServerError> {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs() as i64;

	let mut packet = Packet::empty(clientbound::play::KEEP_ALIVE);
	packet.write_long(timestamp)?;
	client.write_packet(&packet)?;

	let mut packet = client.read_packet(&[serverbound::play::KEEP_ALIVE])?;
	let timestamp2 = packet.read_long()?;
	if timestamp2 != timestamp {
		// Послать клиента нахуй
		Err(ServerError::WrongPacket)
	} else {
		Ok(())
	}
}

pub fn send_system_message(
	client: Arc<ClientContext>,
	message: TextComponent,
	is_action_bar: bool,
) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::SYSTEM_CHAT_MESSAGE);
	packet.write_nbt(&message)?;
	packet.write_boolean(is_action_bar)?;
	client.write_packet(&packet)
}

pub fn unload_chunk(client: Arc<ClientContext>, x: i32, z: i32) -> Result<(), ServerError> {
	let mut packet = Packet::empty(clientbound::play::UNLOAD_CHUNK);
	packet.write_int(z)?;
	packet.write_int(x)?;
	client.write_packet(&packet)
}
