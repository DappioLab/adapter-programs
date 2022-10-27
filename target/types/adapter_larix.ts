export type AdapterLarix = {
  "version": "0.1.0",
  "name": "adapter_larix",
  "instructions": [
    {
      "name": "supply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "unsupply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "borrow",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "repay",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "stake",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "unstake",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "collateralize",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "uncollateralize",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "harvest",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "claimCollateralReward",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    }
  ],
  "types": [
    {
      "name": "SupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenInAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "SupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          },
          {
            "name": "supplyInAmount",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnsupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "UnsupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "supplyOutAmount",
            "type": "u64"
          },
          {
            "name": "reserveInAmount",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BorrowInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "borrowAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BorrowOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenOutAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RepayInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "repayAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RepayOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenInAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "StakeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "StakeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveInAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnstakeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnstakeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "CollateralizeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "CollateralizeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveInAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UncollateralizeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UncollateralizeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "HarvestInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "HarvestOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "ClaimCollateralRewardInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "ClaimCollateralRewardOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedAction",
      "msg": "Unsupported Action"
    }
  ]
};

export const IDL: AdapterLarix = {
  "version": "0.1.0",
  "name": "adapter_larix",
  "instructions": [
    {
      "name": "supply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "unsupply",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "borrow",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "repay",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "stake",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "unstake",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "collateralize",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "uncollateralize",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "harvest",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "claimCollateralReward",
      "accounts": [
        {
          "name": "gatewayAuthority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "gatewayStateInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "baseProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": "bytes"
        }
      ]
    }
  ],
  "types": [
    {
      "name": "SupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenInAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "SupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          },
          {
            "name": "supplyInAmount",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnsupplyInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "UnsupplyOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "supplyOutAmount",
            "type": "u64"
          },
          {
            "name": "reserveInAmount",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BorrowInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "borrowAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "BorrowOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenOutAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RepayInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "repayAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "RepayOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenInAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "StakeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "StakeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveInAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnstakeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UnstakeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "CollateralizeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "CollateralizeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveInAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UncollateralizeInputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "UncollateralizeOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "reserveOutAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "HarvestInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "HarvestOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "ClaimCollateralRewardInputWrapper",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "ClaimCollateralRewardOutputWrapper",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rewardAmount",
            "type": "u64"
          },
          {
            "name": "dummy2",
            "type": "u64"
          },
          {
            "name": "dummy3",
            "type": "u64"
          },
          {
            "name": "dummy4",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "UnsupportedAction",
      "msg": "Unsupported Action"
    }
  ]
};
