#![cfg_attr(not(feature = "std"), no_std)]


/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use codec::{Encode,Decode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, StorageValue, StorageMap, traits::Randomness, Parameter};
use frame_system::ensure_signed;
use sp_runtime::DispatchError;
use sp_io::hashing::blake2_128;
use frame_support::sp_runtime::traits::{Bounded, AtLeast32Bit};
// use frame_support::traits::{ReservableCurrency, Currency};
use frame_support::traits::{Vec, Currency, ReservableCurrency, Get};
use sp_std::vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//已转移到RUNTIME中绑定。
// type KittyIndex =u32;

#[derive(Encode,Decode)]
pub struct Kitty(
	pub [u8;16]
);

//encode 失败,以后尝试一下
//小猫咪的父母信息，ID索引
// #[derive(Encode,Decode)]
// pub struct KittyParent{
// 	pub father:u32,
// 	pub mother:u32,
// }
//
// //小猫咪携带的的信息
// #[derive(Encode,Decode)]
// pub struct KittyInformation{
// 	pub parent:KittyParent,
// 	pub wife:u32,
// 	pub brothers:u32,
// 	pub children:u32,
// }

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

//KittyIndex 实现了 frame_system::Trait 中的类型
pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Randomness: Randomness<Self::Hash>;
	// 定义KittyIndex类型，要求实现指定trait
	// Parameter 表示可用于函数参数传递
	// AtLeast32Bit 表示转换为u32不会造成数据丢失
	// Bounded表示包含上界和下界
	// Default 表示有默认值
	// Copy 表示可以实现Copy 方法
	type KittyIndex: Parameter + AtLeast32Bit + Bounded + Default + Copy;
	//货币类型，用于质押等资产相关的操作
	type NewKittyReserve: Get<BalanceOf<Self>>;
	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}


decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		//结构体映射

		pub Kitties get(fn kitties):map hasher(blake2_128_concat) <T as Trait>::KittyIndex => Option<Kitty>;
		//全局小猫咪数量
		pub KittiesCount get(fn kitties_count):<T as Trait>::KittyIndex;
		//全局小猫咪的持有者信息
		pub KittyOwners get(fn kitty_owner):map hasher(blake2_128_concat) <T as Trait>::KittyIndex => Option<T::AccountId>;
		//持有者自己的小猫咪数量
		pub OwnedKittiesNumber get(fn owned_kitties_number): map hasher(blake2_128_concat) T::AccountId => <T as Trait>::KittyIndex;
		//持有者自己的小猫咪们的序号ID
		pub OwnedKitties get(fn owned_kitties): map hasher(blake2_128_concat) T::AccountId => Vec<<T as Trait>::KittyIndex>;
		// 记录某只猫的父母
		pub KittyParents get(fn kitty_parents):map hasher(blake2_128_concat) <T as Trait>::KittyIndex => (<T as Trait>::KittyIndex, <T as Trait>::KittyIndex);
		// 记录某只猫的对象,可能有好几个哈哈
		pub KittyObject get(fn kitty_object):map hasher(blake2_128_concat) <T as Trait>::KittyIndex => Vec<<T as Trait>::KittyIndex>;
		//记录某只猫的兄弟们
		pub KittyBrother get(fn kitty_brother):map hasher(blake2_128_concat) <T as Trait>::KittyIndex => Vec<<T as Trait>::KittyIndex>;
		//记录某只猫的孩子们
		pub KittyChildren get(fn kitty_children):double_map hasher(blake2_128_concat) <T as Trait>::KittyIndex,  hasher(blake2_128_concat) <T as Trait>::KittyIndex => Vec<<T as Trait>::KittyIndex>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId,
	KittyIndex = <T as Trait>::KittyIndex
	{
		Created(AccountId,KittyIndex),
		Transferred(AccountId,AccountId,KittyIndex),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
		NotKittyOwner,
		NotEnoughMoney,
		KittyNotExists,
		TransferToSelf,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 0]
		    pub fn create(origin){
			let sender = ensure_signed(origin)?;
			let kitty_id =Self::next_kitty_id()?;
			let dna = Self::random_value(&sender);
			let kitty = Kitty(dna);

			T::Currency::reserve(&sender, T::NewKittyReserve::get()).map_err(|_| Error::<T>::NotEnoughMoney)?;

			Self::insert_kitty(&sender,kitty_id,kitty);
			Self::deposit_event(RawEvent::Created(sender,kitty_id));
		}

		#[weight = 0]
			pub fn transfer(origin,to:T::AccountId,kitty_id:<T as Trait>::KittyIndex){
			let sender = ensure_signed(origin)?;
			// //此处一处错误，应验证持有者和宠物索引建立者是否同一人，且有无此宠物ID索引,并且转移后应删除《覆盖》原有的持有者和其宠物索引ID
			// let owner = KittyOwners::<T>::get(kitty_id);
			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			ensure!(sender == owner, Error::<T>::NotKittyOwner);
			// //新增删除,经链上测试无需remove,insert实际上覆盖了持有者的AccountId
			// 不能转让给自己
            ensure!(to != sender, Error::<T>::TransferToSelf);
			// <KittyOwners::<T>>::remove(owner);
			<KittyOwners::<T>>::insert(kitty_id, to.clone());
			Self::deposit_event(RawEvent::Transferred(sender,to,kitty_id));

		}

		#[weight = 0]
		    pub fn breed(origin,kitty_id_1:<T as Trait>::KittyIndex,kitty_id_2:<T as Trait>::KittyIndex){
			let sender = ensure_signed(origin)?;
			let new_kitty_id = Self::do_breed(&sender,kitty_id_1,kitty_id_2)?;
			Self::deposit_event(RawEvent::Created(sender,new_kitty_id));
		}

	}
}

fn combine_dna(dna1:u8,dna2:u8,selector:u8) -> u8{
	(selector & dna1) | (!selector & dna2)
}

impl<T:Trait> Module<T>{
	fn insert_kitty(owner:&T::AccountId,kitty_id:<T as Trait>::KittyIndex,kitty:Kitty){
		//添加结构体Kitties
		<Kitties<T>>::insert(kitty_id,kitty);
		//全局kitties目前的数量《索引》
		<KittiesCount<T>>::put(kitty_id +1.into());
		//全局kitties Owners数据
		<KittyOwners<T>>::insert(kitty_id,owner);
		//持有者自己本身存储添加宠物索引ID,以及数量
		<OwnedKittiesNumber<T>>::insert(owner,kitty_id +1.into());
		let mut number = Self::owned_kitties(owner);
		number.push(kitty_id);
		<OwnedKitties<T>>::insert(owner,number);
	}

	fn next_kitty_id() -> sp_std::result::Result<<T as Trait>::KittyIndex,DispatchError>{
		let kitty_id = Self::kitties_count();
		if kitty_id == <T as Trait>::KittyIndex::max_value(){
			return Err(Error::<T>::KittiesCountOverflow.into())
		}
		Ok(kitty_id)
	}

	fn random_value(sender:&T::AccountId) -> [u8;16]{
		let payload = (
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}


	fn do_breed(sender:&T::AccountId,kitty_id_1:<T as Trait>::KittyIndex,kitty_id_2:<T as Trait>::KittyIndex) -> sp_std::result::Result<<T as Trait>::KittyIndex,DispatchError>{


		//检测`是否同一只猫
		ensure!(kitty_id_1 != kitty_id_2,Error::<T>::RequireDifferentParent);

		// 判断kittyIndex是否存在
		let owner1 = Self::kitty_owner(kitty_id_1).ok_or(Error::<T>::KittyNotExists)?;
		let owner2 = Self::kitty_owner(kitty_id_2).ok_or(Error::<T>::KittyNotExists)?;

		// 判断KittyIndex是否属于发送者
		ensure!(owner1 == *sender, Error::<T>::NotKittyOwner);
		ensure!(owner2 == *sender, Error::<T>::NotKittyOwner);


		// 判断结构体存在
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;
		let selector =Self::random_value(&sender);
		let mut new_dna = [0u8;16];

		for i in 0..kitty1_dna.len(){
			new_dna[i] = combine_dna(kitty1_dna[i],kitty2_dna[i],selector[i]);
		}

		//填入孵化的父母信息
		<KittyParents<T>>::insert(kitty_id,(kitty_id_1,kitty_id_2));
		//填入孵化双方对象信息
		let mut object = Self::kitty_object(kitty_id_1);
		object.push(kitty_id_2);
		<KittyObject<T>>::insert(kitty_id_1,object);

		//----------检测区域
		Self::check_children(kitty_id, kitty_id_1, kitty_id_2);
		Self::check_brother(kitty_id);
		// //填入孵化的孩子信息或者说后代们的信息
		// <KittyChildren<T>>::insert((kitty_id_1,kitty_id_2),kitty_id);
		T::Currency::reserve(&sender, T::NewKittyReserve::get()).map_err(|_| Error::<T>::NotEnoughMoney)?;
		Self::insert_kitty(sender,kitty_id,Kitty(new_dna));
		Ok(kitty_id)
	}
	fn check_children(kitty_id: <T as Trait>::KittyIndex, kitty_id_1: <T as Trait>::KittyIndex, kitty_id_2: <T as Trait>::KittyIndex,){
		//检测到有孵化者,也就是值为TRUE则插入值,填入新的兄弟索引号,否则相当于插入孵化的第一个孩子大哥大哈哈
		if <KittyChildren<T>>::contains_key(kitty_id_1, kitty_id_2) {
			<KittyChildren<T>>::mutate(kitty_id_1, kitty_id_2, |val| val.push(kitty_id));
		} else {
			// 如果不存在 重新插入一个孵化双方和孩子
			<KittyChildren<T>>::insert(kitty_id_1, kitty_id_2, vec![kitty_id]);
		}
	}

	fn check_brother(kitty_id: <T as Trait>::KittyIndex){
		//先把指定猫索引的父母找出
		let (kitty_id_1, kitty_id_2) = <KittyParents<T>>::get(kitty_id);
		//如果其中一只猫的后代有kitty_id,则继续执行迭代器反之填入当前后代
			if <KittyChildren<T>>::contains_key(kitty_id_1, kitty_id_2) {
				let val: Vec<<T as Trait>::KittyIndex> = <KittyChildren<T>>::get(kitty_id_1, kitty_id_2);
				let reserve_val: Vec<<T as Trait>::KittyIndex> =
					val.into_iter().filter(|&val| val != kitty_id).collect();
				<KittyBrother<T>>::insert(kitty_id, reserve_val);
			} else {
				<KittyBrother<T>>::insert(kitty_id, vec::Vec::<T::KittyIndex>::new());
			}
		}

}



