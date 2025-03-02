import React from "react";
import { toHex, zeroAddress } from "viem";

import { useZrpGetClaimId } from "../generated";
import tokens from "../assets/tokens.json";
import { Balance } from "./Balance";
import { Token } from "../libs/types";

export type TokenData = {
  name: string;
  icon: string;
  address: `0x${string}`;
  decimals: number;
};

interface AccountProps {
  email: string | null;
  disabled: boolean;
  hideClaim?: boolean;
}

const Account: React.FC<AccountProps> = (props) => {
  const { email, disabled, hideClaim } = props;

  const { data: ethClaimId } = useZrpGetClaimId({
    args: [toHex(email ?? ""), tokens["sepolia"][0].address as `0x${string}`],
  });

  const { data: usdcClaimId } = useZrpGetClaimId({
    args: [toHex(email ?? ""), tokens["sepolia"][1].address as `0x${string}`],
  });

  return (
    <>
      <div className="balance-container">
        <Balance
          identity={ethClaimId ?? zeroAddress}
          token={tokens["sepolia"][0] as Token}
          disabled={disabled}
          hideClaim={hideClaim}
        />
        <Balance
          identity={usdcClaimId ?? zeroAddress}
          token={tokens["sepolia"][1] as Token}
          disabled={disabled}
          hideClaim={hideClaim}
        />
      </div>
    </>
  );
};

export default Account;
