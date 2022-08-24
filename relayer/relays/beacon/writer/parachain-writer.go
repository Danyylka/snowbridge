package writer

import (
	"context"
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	"github.com/snowfork/go-substrate-rpc-client/v4/types"
	"github.com/snowfork/snowbridge/relayer/chain/parachain"
	"golang.org/x/sync/errgroup"
)

type ParachainWriter struct {
	conn        *parachain.Connection
	nonce       uint32
	pool        *parachain.ExtrinsicPool
	genesisHash types.Hash
}

func NewParachainWriter(
	conn *parachain.Connection,
) *ParachainWriter {
	return &ParachainWriter{
		conn: conn,
	}
}

func (wr *ParachainWriter) Start(ctx context.Context, eg *errgroup.Group) error {
	nonce, err := wr.queryAccountNonce()
	if err != nil {
		return err
	}
	wr.nonce = nonce

	genesisHash, err := wr.conn.API().RPC.Chain.GetBlockHash(0)
	if err != nil {
		return err
	}
	wr.genesisHash = genesisHash

	wr.pool = parachain.NewExtrinsicPool(eg, wr.conn)

	return nil
}

func (wr *ParachainWriter) queryAccountNonce() (uint32, error) {
	key, err := types.CreateStorageKey(wr.conn.Metadata(), "System", "Account", wr.conn.Keypair().PublicKey, nil)
	if err != nil {
		return 0, err
	}

	var accountInfo types.AccountInfo
	ok, err := wr.conn.API().RPC.State.GetStorageLatest(key, &accountInfo)
	if err != nil {
		return 0, err
	}
	if !ok {
		return 0, fmt.Errorf("no account info found for %s", wr.conn.Keypair().URI)
	}

	return uint32(accountInfo.Nonce), nil
}

func (wr *ParachainWriter) WriteToParachain(ctx context.Context, extrinsicName string, payload ...interface{}) error {
	meta, err := wr.conn.API().RPC.State.GetMetadataLatest()
	if err != nil {
		return err
	}

	c, err := types.NewCall(meta, extrinsicName, payload...)
	if err != nil {
		return err
	}

	latestHash, err := wr.conn.API().RPC.Chain.GetFinalizedHead()
	if err != nil {
		return err
	}

	latestBlock, err := wr.conn.API().RPC.Chain.GetBlock(latestHash)
	if err != nil {
		return err
	}

	ext := types.NewExtrinsic(c)
	era := parachain.NewMortalEra(uint64(latestBlock.Block.Header.Number))

	genesisHash, err := wr.conn.API().RPC.Chain.GetBlockHash(0)
	if err != nil {
		return err
	}

	rv, err := wr.conn.API().RPC.State.GetRuntimeVersionLatest()
	if err != nil {
		return err
	}

	o := types.SignatureOptions{
		BlockHash:          latestHash,
		Era:                era,
		GenesisHash:        genesisHash,
		Nonce:              types.NewUCompactFromUInt(uint64(wr.nonce)),
		SpecVersion:        rv.SpecVersion,
		Tip:                types.NewUCompactFromUInt(0),
		TransactionVersion: rv.TransactionVersion,
	}

	extI := ext

	err = extI.Sign(*wr.conn.Keypair(), o)
	if err != nil {
		return err
	}

	_, err = wr.conn.API().RPC.Author.SubmitAndWatchExtrinsic(extI)
	if err != nil {
		return err
	}

	wr.nonce = wr.nonce + 1

	return nil
}

func (wr *ParachainWriter) GetLastSyncedSyncCommitteePeriod() (uint64, error) {
	key, err := types.CreateStorageKey(wr.conn.Metadata(), "EthereumBeaconClient", "LatestSyncCommitteePeriod", nil, nil)
	if err != nil {
		return 0, fmt.Errorf("create storage key for last sync committee: %w", err)
	}

	var period types.U64
	_, err = wr.conn.API().RPC.State.GetStorageLatest(key, &period)
	if err != nil {
		return 0, fmt.Errorf("get storage for latest synced sync committee period (err): %w", err)
	}

	return uint64(period), nil
}

func (wr *ParachainWriter) GetLastStoredFinalizedHeader() (common.Hash, error) {
	key, err := types.CreateStorageKey(wr.conn.Metadata(), "EthereumBeaconClient", "LatestFinalizedHeaderHash", nil, nil)
	if err != nil {
		return common.Hash{}, fmt.Errorf("create storage key for last finalized header hash: %w", err)
	}

	var hash types.H256
	_, err = wr.conn.API().RPC.State.GetStorageLatest(key, &hash)
	if err != nil {
		return common.Hash{}, fmt.Errorf("get storage for latest finalized header hash (err): %w", err)
	}

	return common.HexToHash(hash.Hex()), nil
}

func (wr *ParachainWriter) GetLastStoredFinalizedHeaderSlot() (uint64, error) {
	key, err := types.CreateStorageKey(wr.conn.Metadata(), "EthereumBeaconClient", "LatestFinalizedHeaderSlot", nil, nil)
	if err != nil {
		return 0, fmt.Errorf("create storage key for last finalized header slot: %w", err)
	}

	var slot types.U64
	_, err = wr.conn.API().RPC.State.GetStorageLatest(key, &slot)
	if err != nil {
		return 0, fmt.Errorf("get storage for latest finalized header slot (err): %w", err)
	}

	return uint64(slot), nil
}

func (wr *ParachainWriter) GetLastBasicChannelMessage() (uint64, error) {
	key, err := types.CreateStorageKey(wr.conn.Metadata(), "BasicInboundChannel", "LatestVerifiedBlockNumber", nil, nil)
	if err != nil {
		return 0, fmt.Errorf("create storage key for last sync committee: %w", err)
	}

	var blockNumber types.U64
	_, err = wr.conn.API().RPC.State.GetStorageLatest(key, &blockNumber)
	if err != nil {
		return 0, fmt.Errorf("get storage for latest synced sync committee period (err): %w", err)
	}

	return uint64(blockNumber), nil
}

func (wr *ParachainWriter) GetLastBasicChannelNonce() (uint64, error) {
	key, err := types.CreateStorageKey(wr.conn.Metadata(), "BasicInboundChannel", "Nonce", nil, nil)
	if err != nil {
		return 0, fmt.Errorf("create storage key for last sync committee: %w", err)
	}

	var nonce types.U64
	_, err = wr.conn.API().RPC.State.GetStorageLatest(key, &nonce)
	if err != nil {
		return 0, fmt.Errorf("get storage for latest synced sync committee period (err): %w", err)
	}

	return uint64(nonce), nil
}

func (wr *ParachainWriter) GetLastIncentivizedChannelMessage() (uint64, error) {
	key, err := types.CreateStorageKey(wr.conn.Metadata(), "IncentivizedInboundChannel", "LatestVerifiedBlockNumber", nil, nil)
	if err != nil {
		return 0, fmt.Errorf("create storage key for last sync committee: %w", err)
	}

	var blockNumber types.U64
	_, err = wr.conn.API().RPC.State.GetStorageLatest(key, &blockNumber)
	if err != nil {
		return 0, fmt.Errorf("get storage for latest synced sync committee period (err): %w", err)
	}

	return uint64(blockNumber), nil
}

func (wr *ParachainWriter) GetLastIncentivizedChannelNonce() (uint64, error) {
	key, err := types.CreateStorageKey(wr.conn.Metadata(), "IncentivizedInboundChannel", "Nonce", nil, nil)
	if err != nil {
		return 0, fmt.Errorf("create storage key for last sync committee: %w", err)
	}

	var nonce types.U64
	_, err = wr.conn.API().RPC.State.GetStorageLatest(key, &nonce)
	if err != nil {
		return 0, fmt.Errorf("get storage for latest synced sync committee period (err): %w", err)
	}

	return uint64(nonce), nil
}